//! Writing article files into the site repository with the GitHub
//! Contents API.
//!
//! The token is a fine-grained personal access token limited to one
//! repository with "Contents: read and write". It is a worker secret and
//! never reaches the browser.

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use serde::{Deserialize, Serialize};
use worker::wasm_bindgen::JsValue;
use worker::{Env, Fetch, Headers, Method, Request, RequestInit};

use crate::http::ApiError;

const USER_AGENT: &str = "nows-portfolio-cms";

pub struct Repo {
    /// `owner/name`
    pub full_name: String,
    pub branch: String,
    pub content_dir: String,
    token: String,
}

impl Repo {
    pub fn from_env(env: &Env) -> Result<Self, ApiError> {
        let var = |name: &str| {
            env.var(name)
                .map(|v| v.to_string())
                .map_err(|_| ApiError::internal(format!("{name} is not set in wrangler.toml.")))
        };
        let token = env
            .secret("GITHUB_CONTENT_TOKEN")
            .map(|s| s.to_string())
            .map_err(|_| ApiError::internal("GITHUB_CONTENT_TOKEN is not set. Run `npx wrangler secret put GITHUB_CONTENT_TOKEN`."))?;
        let full_name = var("GITHUB_REPO")?;
        if full_name.split('/').count() != 2 || full_name.contains("..") {
            return Err(ApiError::internal("GITHUB_REPO must look like owner/name."));
        }
        Ok(Self {
            full_name,
            branch: var("GITHUB_BRANCH").unwrap_or_else(|_| "main".into()),
            content_dir: var("CONTENT_DIR").unwrap_or_else(|_| "site/content/articles".into()),
            token,
        })
    }

    fn contents_url(&self, path: &str) -> String {
        format!(
            "{}/repos/{}/contents/{}",
            cms_core::oauth::GITHUB_API,
            self.full_name,
            path
        )
    }

    fn headers(&self) -> Result<Headers, ApiError> {
        let headers = Headers::new();
        headers.set("Accept", "application/vnd.github+json")?;
        headers.set("User-Agent", USER_AGENT)?;
        headers.set("X-GitHub-Api-Version", "2022-11-28")?;
        headers.set("Authorization", &format!("Bearer {}", self.token))?;
        Ok(headers)
    }

    async fn send(
        &self,
        method: Method,
        url: &str,
        body: Option<String>,
    ) -> Result<worker::Response, ApiError> {
        let headers = self.headers()?;
        if body.is_some() {
            headers.set("Content-Type", "application/json")?;
        }
        let mut init = RequestInit::new();
        init.with_method(method).with_headers(headers);
        if let Some(b) = body {
            init.with_body(Some(JsValue::from_str(&b)));
        }
        let request = Request::new_with_init(url, &init)?;
        Ok(Fetch::Request(request).send().await?)
    }

    /// The blob sha of a file, or `None` when it does not exist yet.
    pub async fn file_sha(&self, path: &str) -> Result<Option<String>, ApiError> {
        #[derive(Deserialize)]
        struct Content {
            sha: String,
        }
        let url = format!("{}?ref={}", self.contents_url(path), self.branch);
        let mut response = self.send(Method::Get, &url, None).await?;
        match response.status_code() {
            200 => Ok(Some(response.json::<Content>().await?.sha)),
            404 => Ok(None),
            code => Err(github_error(code, &mut response).await),
        }
    }

    /// Create or replace a file. Returns the new blob sha.
    pub async fn put_file(
        &self,
        path: &str,
        content: &str,
        message: &str,
    ) -> Result<String, ApiError> {
        #[derive(Serialize)]
        struct Body<'a> {
            message: &'a str,
            content: String,
            branch: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            sha: Option<String>,
        }
        #[derive(Deserialize)]
        struct Reply {
            content: Content,
        }
        #[derive(Deserialize)]
        struct Content {
            sha: String,
        }

        let body = Body {
            message,
            content: STANDARD.encode(content.as_bytes()),
            branch: &self.branch,
            sha: self.file_sha(path).await?,
        };
        let json = serde_json::to_string(&body).map_err(ApiError::internal)?;
        let mut response = self
            .send(Method::Put, &self.contents_url(path), Some(json))
            .await?;
        match response.status_code() {
            200 | 201 => Ok(response.json::<Reply>().await?.content.sha),
            code => Err(github_error(code, &mut response).await),
        }
    }

    /// Delete a file if it exists.
    pub async fn delete_file(&self, path: &str, message: &str) -> Result<(), ApiError> {
        let Some(sha) = self.file_sha(path).await? else {
            return Ok(());
        };
        let body = serde_json::json!({ "message": message, "sha": sha, "branch": self.branch })
            .to_string();
        let mut response = self
            .send(Method::Delete, &self.contents_url(path), Some(body))
            .await?;
        match response.status_code() {
            200 => Ok(()),
            code => Err(github_error(code, &mut response).await),
        }
    }
}

async fn github_error(code: u16, response: &mut worker::Response) -> ApiError {
    let detail = response.text().await.unwrap_or_default();
    worker::console_error!("GitHub API {code}: {detail}");
    let message = match code {
        401 => {
            "GitHub rejected the content token. Create a new one and update GITHUB_CONTENT_TOKEN."
        }
        403 => {
            "The content token cannot write to this repository. Give it Contents: read and write."
        }
        404 => "GitHub cannot find the repository or branch. Check GITHUB_REPO and GITHUB_BRANCH.",
        409 => "The file changed on GitHub at the same moment. Publish again.",
        422 => "GitHub refused the file. Check the worker logs for the reason.",
        _ => "GitHub is not answering properly. Try again in a minute.",
    };
    ApiError::new(502, "github", message)
}
