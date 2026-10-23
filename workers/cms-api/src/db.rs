//! All D1 queries live here. Every value goes in through `bind`, never by
//! formatting it into the SQL string, so SQL injection is not possible.

use serde::{Deserialize, Serialize};
use worker::wasm_bindgen::JsValue;
use worker::{D1Database, Env};

use crate::http::ApiError;

pub fn database(env: &Env) -> Result<D1Database, ApiError> {
    env.d1("DB")
        .map_err(|_| ApiError::internal("D1 binding DB is missing. Check wrangler.toml."))
}

/// One row of the `articles` table, exactly as stored.
#[derive(Debug, Clone, Deserialize)]
pub struct ArticleRow {
    pub id: String,
    pub slug: String,
    pub title: String,
    pub description: String,
    /// JSON array of strings.
    pub tags: String,
    pub body: String,
    pub date: Option<String>,
    pub status: String,
    pub version: i64,
    pub created_at: i64,
    pub updated_at: i64,
    pub published_at: Option<i64>,
    pub published_slug: Option<String>,
    pub published_sha: Option<String>,
    pub updated_by: String,
}

impl ArticleRow {
    pub fn tags(&self) -> Vec<String> {
        serde_json::from_str(&self.tags).unwrap_or_default()
    }

    /// The shape the admin editor works with.
    pub fn to_admin(&self) -> AdminArticle {
        AdminArticle {
            id: self.id.clone(),
            slug: self.slug.clone(),
            title: self.title.clone(),
            description: self.description.clone(),
            tags: self.tags(),
            body: self.body.clone(),
            date: self.date.clone().unwrap_or_default(),
            status: self.status.clone(),
            version: self.version,
            created_at: self.created_at,
            updated_at: self.updated_at,
            published_at: self.published_at,
            published_slug: self.published_slug.clone(),
            published_sha: self.published_sha.clone(),
            updated_by: self.updated_by.clone(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AdminArticle {
    pub id: String,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub body: String,
    pub date: String,
    pub status: String,
    pub version: i64,
    pub created_at: i64,
    pub updated_at: i64,
    pub published_at: Option<i64>,
    pub published_slug: Option<String>,
    pub published_sha: Option<String>,
    pub updated_by: String,
}

/// A short row for the admin list.
#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleSummary {
    pub id: String,
    pub slug: String,
    pub title: String,
    pub status: String,
    pub updated_at: i64,
    pub published_at: Option<i64>,
    pub updated_by: String,
}

/// What the public API shows. No ids, no drafts, no editor names.
#[derive(Debug, Serialize, Deserialize)]
pub struct PublicArticle {
    pub slug: String,
    pub title: String,
    pub description: String,
    #[serde(skip_serializing)]
    pub tags: String,
    pub date: Option<String>,
    pub published_at: Option<i64>,
}

const COLUMNS: &str = "id, slug, title, description, tags, body, date, status, version, created_at, updated_at, published_at, published_slug, published_sha, updated_by";

fn text(s: &str) -> JsValue {
    JsValue::from_str(s)
}

fn opt_text(s: Option<&str>) -> JsValue {
    s.map(JsValue::from_str).unwrap_or(JsValue::NULL)
}

fn int(n: i64) -> JsValue {
    // D1 numbers are JavaScript numbers. Our values (Unix seconds, small
    // counters) are far below 2^53, so the conversion is exact.
    JsValue::from_f64(n as f64)
}

pub async fn list_all(db: &D1Database) -> Result<Vec<ArticleSummary>, ApiError> {
    let result = db
        .prepare("SELECT id, slug, title, status, updated_at, published_at, updated_by FROM articles ORDER BY updated_at DESC LIMIT 500")
        .all()
        .await?;
    Ok(result.results()?)
}

pub async fn list_published(db: &D1Database, limit: u32) -> Result<Vec<PublicArticle>, ApiError> {
    let result = db
        .prepare("SELECT slug, title, description, tags, date, published_at FROM articles WHERE status = 'published' ORDER BY published_at DESC LIMIT ?1")
        .bind(&[int(i64::from(limit))])?
        .all()
        .await?;
    Ok(result.results()?)
}

pub async fn get(db: &D1Database, id: &str) -> Result<Option<ArticleRow>, ApiError> {
    let sql = format!("SELECT {COLUMNS} FROM articles WHERE id = ?1");
    Ok(db.prepare(sql).bind(&[text(id)])?.first(None).await?)
}

pub async fn get_published_by_slug(
    db: &D1Database,
    slug: &str,
) -> Result<Option<ArticleRow>, ApiError> {
    let sql = format!("SELECT {COLUMNS} FROM articles WHERE slug = ?1 AND status = 'published'");
    Ok(db.prepare(sql).bind(&[text(slug)])?.first(None).await?)
}

pub async fn slug_taken(
    db: &D1Database,
    slug: &str,
    except_id: Option<&str>,
) -> Result<bool, ApiError> {
    #[derive(Deserialize)]
    struct Count {
        n: i64,
    }
    let row: Option<Count> = db
        .prepare("SELECT COUNT(*) AS n FROM articles WHERE slug = ?1 AND id != ?2")
        .bind(&[text(slug), text(except_id.unwrap_or(""))])?
        .first(None)
        .await?;
    Ok(row.map(|c| c.n > 0).unwrap_or(false))
}

pub struct NewArticle<'a> {
    pub id: &'a str,
    pub slug: &'a str,
    pub title: &'a str,
    pub description: &'a str,
    pub tags_json: &'a str,
    pub body: &'a str,
    pub date: Option<&'a str>,
    pub now: i64,
    pub by: &'a str,
}

pub async fn insert(db: &D1Database, a: &NewArticle<'_>) -> Result<(), ApiError> {
    db.prepare(
        "INSERT INTO articles (id, slug, title, description, tags, body, date, status, version, created_at, updated_at, updated_by)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 'draft', 1, ?8, ?8, ?9)",
    )
    .bind(&[
        text(a.id),
        text(a.slug),
        text(a.title),
        text(a.description),
        text(a.tags_json),
        text(a.body),
        opt_text(a.date),
        int(a.now),
        text(a.by),
    ])?
    .run()
    .await?;
    Ok(())
}

/// Update the editable fields, but only if nobody saved in between.
/// Returns `false` when the stored version moved on (someone else saved).
pub async fn update(
    db: &D1Database,
    a: &NewArticle<'_>,
    expected_version: i64,
) -> Result<bool, ApiError> {
    let result = db
        .prepare(
            "UPDATE articles SET slug = ?2, title = ?3, description = ?4, tags = ?5, body = ?6, date = ?7,
                 version = version + 1, updated_at = ?8, updated_by = ?9
             WHERE id = ?1 AND version = ?10",
        )
        .bind(&[
            text(a.id),
            text(a.slug),
            text(a.title),
            text(a.description),
            text(a.tags_json),
            text(a.body),
            opt_text(a.date),
            int(a.now),
            text(a.by),
            int(expected_version),
        ])?
        .run()
        .await?;
    Ok(changes(&result) > 0)
}

pub async fn mark_published(
    db: &D1Database,
    id: &str,
    slug: &str,
    sha: &str,
    now: i64,
    by: &str,
) -> Result<(), ApiError> {
    db.prepare(
        "UPDATE articles SET status = 'published', published_at = COALESCE(published_at, ?2), published_slug = ?3,
             published_sha = ?4, version = version + 1, updated_at = ?2, updated_by = ?5
         WHERE id = ?1",
    )
    .bind(&[text(id), int(now), text(slug), text(sha), text(by)])?
    .run()
    .await?;
    Ok(())
}

pub async fn mark_draft(
    db: &D1Database,
    id: &str,
    sha: &str,
    now: i64,
    by: &str,
) -> Result<(), ApiError> {
    db.prepare(
        "UPDATE articles SET status = 'draft', published_sha = ?2, version = version + 1, updated_at = ?3, updated_by = ?4
         WHERE id = ?1",
    )
    .bind(&[text(id), text(sha), int(now), text(by)])?
    .run()
    .await?;
    Ok(())
}

pub async fn delete(db: &D1Database, id: &str) -> Result<(), ApiError> {
    db.prepare("DELETE FROM articles WHERE id = ?1")
        .bind(&[text(id)])?
        .run()
        .await?;
    Ok(())
}

pub async fn audit(
    db: &D1Database,
    now: i64,
    actor: &str,
    action: &str,
    article_id: Option<&str>,
    detail: &str,
) -> Result<(), ApiError> {
    db.prepare(
        "INSERT INTO audit_log (at, actor, action, article_id, detail) VALUES (?1, ?2, ?3, ?4, ?5)",
    )
    .bind(&[
        int(now),
        text(actor),
        text(action),
        opt_text(article_id),
        text(detail),
    ])?
    .run()
    .await?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditEntry {
    pub at: i64,
    pub actor: String,
    pub action: String,
    pub article_id: Option<String>,
    pub detail: String,
}

pub async fn audit_recent(db: &D1Database) -> Result<Vec<AuditEntry>, ApiError> {
    let result = db
        .prepare("SELECT at, actor, action, article_id, detail FROM audit_log ORDER BY at DESC, id DESC LIMIT 100")
        .all()
        .await?;
    Ok(result.results()?)
}

fn changes(result: &worker::D1Result) -> usize {
    result
        .meta()
        .ok()
        .flatten()
        .and_then(|m| m.changes)
        .unwrap_or(0)
}
