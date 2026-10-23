//! Article handlers for the public and admin APIs.

use cms_core::frontmatter::{self, FrontMatter};
use cms_core::{preview, validate, ArticleInput, Date, Status, ValidArticle};
use serde::Deserialize;
use worker::{Env, Method, Request, Url};

use crate::auth::{now_secs, random_hex, User};
use crate::db::{self, NewArticle};
use crate::github::Repo;
use crate::http::{json, site_origin, ApiError, ApiResult};

/// Largest request body we read. The article limit is 200 KB of Markdown;
/// JSON escaping can make that bigger, so allow some headroom.
const MAX_REQUEST_BYTES: usize = 400_000;

// ---------- public, read-only ----------

pub async fn public_list(url: &Url, env: &Env) -> ApiResult {
    let limit = url
        .query_pairs()
        .find(|(k, _)| k == "limit")
        .and_then(|(_, v)| v.parse::<u32>().ok())
        .unwrap_or(20)
        .clamp(1, 50);
    let rows = db::list_published(&db::database(env)?, limit).await?;
    let articles: Vec<_> = rows
        .iter()
        .map(|r| {
            serde_json::json!({
                "slug": r.slug,
                "title": r.title,
                "description": r.description,
                "tags": serde_json::from_str::<Vec<String>>(&r.tags).unwrap_or_default(),
                "date": r.date,
                "published_at": r.published_at,
            })
        })
        .collect();
    with_cors(
        json(200, &serde_json::json!({ "articles": articles }))?,
        env,
    )
}

pub async fn public_get(slug: &str, env: &Env) -> ApiResult {
    if !cms_core::slug::is_valid_slug(slug) {
        return Err(ApiError::not_found(
            "No published article with that address.",
        ));
    }
    let row = db::get_published_by_slug(&db::database(env)?, slug)
        .await?
        .ok_or_else(|| ApiError::not_found("No published article with that address."))?;
    let body = serde_json::json!({
        "slug": row.slug,
        "title": row.title,
        "description": row.description,
        "tags": row.tags(),
        "date": row.date,
        "published_at": row.published_at,
        "body": row.body,
    });
    with_cors(json(200, &body)?, env)
}

fn with_cors(mut response: worker::Response, env: &Env) -> ApiResult {
    let origin = site_origin(env);
    if !origin.is_empty() {
        response
            .headers_mut()
            .set("Access-Control-Allow-Origin", &origin)?;
        response.headers_mut().set("Vary", "Origin")?;
    }
    // Public data can be cached briefly at the edge.
    response
        .headers_mut()
        .set("Cache-Control", "public, max-age=60")?;
    Ok(response)
}

// ---------- admin ----------

pub async fn admin(
    method: Method,
    rest: &[String],
    req: Request,
    env: &Env,
    user: &User,
) -> ApiResult {
    let parts: Vec<&str> = rest.iter().map(String::as_str).collect();
    match (method, parts.as_slice()) {
        (Method::Get, ["articles"]) => list(env).await,
        (Method::Post, ["articles"]) => create(req, env, user).await,
        (Method::Get, ["articles", id]) => get(id, env).await,
        (Method::Put, ["articles", id]) => update(id, req, env, user).await,
        (Method::Delete, ["articles", id]) => delete(id, env, user).await,
        (Method::Post, ["articles", id, "publish"]) => publish(id, env, user).await,
        (Method::Post, ["articles", id, "unpublish"]) => unpublish(id, env, user).await,
        (Method::Post, ["preview"]) => render_preview(req).await,
        (Method::Get, ["audit"]) => audit(env).await,
        _ => Err(ApiError::not_found("No such admin route.")),
    }
}

async fn read_json<T: for<'de> Deserialize<'de>>(mut req: Request) -> Result<T, ApiError> {
    if let Some(length) = req.headers().get("Content-Length")? {
        if length.parse::<usize>().unwrap_or(usize::MAX) > MAX_REQUEST_BYTES {
            return Err(ApiError::new(
                413,
                "too_large",
                "That request is too large.",
            ));
        }
    }
    let text = req.text().await?;
    if text.len() > MAX_REQUEST_BYTES {
        return Err(ApiError::new(
            413,
            "too_large",
            "That request is too large.",
        ));
    }
    serde_json::from_str(&text)
        .map_err(|e| ApiError::bad_request(format!("The request body is not valid JSON: {e}")))
}

fn valid_id(id: &str) -> Result<(), ApiError> {
    if id.len() == 24 && id.bytes().all(|b| b.is_ascii_hexdigit()) {
        Ok(())
    } else {
        Err(ApiError::not_found("No article with that id."))
    }
}

async fn load(id: &str, env: &Env) -> Result<db::ArticleRow, ApiError> {
    valid_id(id)?;
    db::get(&db::database(env)?, id)
        .await?
        .ok_or_else(|| ApiError::not_found("No article with that id."))
}

async fn list(env: &Env) -> ApiResult {
    let articles = db::list_all(&db::database(env)?).await?;
    json(200, &serde_json::json!({ "articles": articles }))
}

async fn get(id: &str, env: &Env) -> ApiResult {
    let row = load(id, env).await?;
    json(200, &serde_json::json!({ "article": row.to_admin() }))
}

fn tags_json(article: &ValidArticle) -> String {
    serde_json::to_string(&article.tags).unwrap_or_else(|_| "[]".into())
}

async fn create(req: Request, env: &Env, user: &User) -> ApiResult {
    let input: ArticleInput = read_json(req).await?;
    let article = validate(input).map_err(ApiError::invalid)?;
    let database = db::database(env)?;
    if db::slug_taken(&database, &article.slug, None).await? {
        return Err(ApiError::field(
            "slug",
            "Another article already uses this slug.",
        ));
    }

    let id = random_hex(12)?;
    let now = now_secs() as i64;
    let date = article.date.map(|d| d.to_string());
    let tags = tags_json(&article);
    db::insert(
        &database,
        &NewArticle {
            id: &id,
            slug: &article.slug,
            title: &article.title,
            description: &article.description,
            tags_json: &tags,
            body: &article.body,
            date: date.as_deref(),
            now,
            by: &user.login,
        },
    )
    .await?;
    db::audit(
        &database,
        now,
        &user.login,
        "create",
        Some(&id),
        &article.title,
    )
    .await?;

    let row = load(&id, env).await?;
    json(201, &serde_json::json!({ "article": row.to_admin() }))
}

#[derive(Deserialize)]
struct UpdateRequest {
    #[serde(flatten)]
    input: ArticleInput,
    version: i64,
}

async fn update(id: &str, req: Request, env: &Env, user: &User) -> ApiResult {
    let existing = load(id, env).await?;
    let UpdateRequest { input, version } = read_json(req).await?;
    let article = validate(input).map_err(ApiError::invalid)?;

    if let Some(published_slug) = &existing.published_slug {
        if &article.slug != published_slug {
            return Err(ApiError::field(
                "slug",
                format!("This article has been published at /articles/{published_slug}/, so its slug is fixed. Changing it would break existing links."),
            ));
        }
    }

    let database = db::database(env)?;
    if db::slug_taken(&database, &article.slug, Some(id)).await? {
        return Err(ApiError::field(
            "slug",
            "Another article already uses this slug.",
        ));
    }

    let now = now_secs() as i64;
    let date = article.date.map(|d| d.to_string());
    let tags = tags_json(&article);
    let saved = db::update(
        &database,
        &NewArticle {
            id,
            slug: &article.slug,
            title: &article.title,
            description: &article.description,
            tags_json: &tags,
            body: &article.body,
            date: date.as_deref(),
            now,
            by: &user.login,
        },
        version,
    )
    .await?;
    if !saved {
        return Err(ApiError::conflict(
            "This article was saved somewhere else after you opened it. Copy your changes, reload, and apply them again.",
        ));
    }
    db::audit(
        &database,
        now,
        &user.login,
        "update",
        Some(id),
        &article.title,
    )
    .await?;

    let row = load(id, env).await?;
    let mut note = serde_json::Value::Null;
    if row.status == Status::Published.as_str() {
        note = "Saved. Publish again to update the live page.".into();
    }
    json(
        200,
        &serde_json::json!({ "article": row.to_admin(), "note": note }),
    )
}

/// Re-check a stored row before it leaves for git, in case the rules got
/// stricter since it was saved.
fn revalidate(row: &db::ArticleRow) -> Result<ValidArticle, ApiError> {
    validate(ArticleInput {
        title: row.title.clone(),
        slug: row.slug.clone(),
        description: row.description.clone(),
        tags: row.tags(),
        body: row.body.clone(),
        date: row.date.clone().unwrap_or_default(),
    })
    .map_err(ApiError::invalid)
}

fn render_file(row: &db::ArticleRow, article: &ValidArticle, draft: bool, now: i64) -> String {
    let today = Date::from_unix(now);
    let first_published = row.published_at.map(Date::from_unix).unwrap_or(today);
    let updated = row.published_at.map(|_| today);
    let front = FrontMatter::for_article(article, first_published, updated, draft);
    frontmatter::render(&front, &article.body)
}

async fn publish(id: &str, env: &Env, user: &User) -> ApiResult {
    let row = load(id, env).await?;
    let article = revalidate(&row)?;
    let repo = Repo::from_env(env)?;
    let now = now_secs() as i64;

    let file = render_file(&row, &article, false, now);
    let path = frontmatter::article_path(&repo.content_dir, &article.slug);
    let verb = if row.published_at.is_some() {
        "Update"
    } else {
        "Publish"
    };
    let sha = repo
        .put_file(&path, &file, &format!("{verb} article: {}", article.title))
        .await?;

    let database = db::database(env)?;
    db::mark_published(&database, id, &article.slug, &sha, now, &user.login).await?;
    db::audit(&database, now, &user.login, "publish", Some(id), &path).await?;

    let row = load(id, env).await?;
    json(
        200,
        &serde_json::json!({
            "article": row.to_admin(),
            "note": "Committed to GitHub. The site rebuilds in about a minute.",
        }),
    )
}

async fn unpublish(id: &str, env: &Env, user: &User) -> ApiResult {
    let row = load(id, env).await?;
    if row.status != Status::Published.as_str() {
        return Err(ApiError::conflict("This article is not published."));
    }
    let article = revalidate(&row)?;
    let repo = Repo::from_env(env)?;
    let now = now_secs() as i64;

    // We keep the file and mark it `draft = true`, so Zola stops building
    // the page while git keeps its history.
    let file = render_file(&row, &article, true, now);
    let path = frontmatter::article_path(&repo.content_dir, &article.slug);
    let sha = repo
        .put_file(
            &path,
            &file,
            &format!("Unpublish article: {}", article.title),
        )
        .await?;

    let database = db::database(env)?;
    db::mark_draft(&database, id, &sha, now, &user.login).await?;
    db::audit(&database, now, &user.login, "unpublish", Some(id), &path).await?;

    let row = load(id, env).await?;
    json(
        200,
        &serde_json::json!({ "article": row.to_admin(), "note": "The page is hidden after the next site build." }),
    )
}

async fn delete(id: &str, env: &Env, user: &User) -> ApiResult {
    let row = load(id, env).await?;
    if row.status == Status::Published.as_str() {
        return Err(ApiError::conflict(
            "Unpublish this article before deleting it.",
        ));
    }
    let database = db::database(env)?;
    let now = now_secs() as i64;
    if let Some(published_slug) = &row.published_slug {
        let repo = Repo::from_env(env)?;
        let path = frontmatter::article_path(&repo.content_dir, published_slug);
        repo.delete_file(&path, &format!("Delete article: {}", row.title))
            .await?;
    }
    db::delete(&database, id).await?;
    db::audit(&database, now, &user.login, "delete", Some(id), &row.title).await?;
    Ok(worker::Response::empty()?.with_status(204))
}

#[derive(Deserialize)]
struct PreviewRequest {
    markdown: String,
}

async fn render_preview(req: Request) -> ApiResult {
    let PreviewRequest { markdown } = read_json(req).await?;
    json(
        200,
        &serde_json::json!({
            "html": preview::render_safe(&markdown),
            "minutes": preview::reading_minutes(&markdown),
        }),
    )
}

async fn audit(env: &Env) -> ApiResult {
    let entries = db::audit_recent(&db::database(env)?).await?;
    json(200, &serde_json::json!({ "entries": entries }))
}
