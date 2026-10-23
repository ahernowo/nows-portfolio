//! Portfolio CMS API (stage B).
//!
//! Drafts live in Cloudflare D1. Publishing writes a Markdown file into the
//! site's git repository through the GitHub API, and Cloudflare Pages then
//! rebuilds the static site. Git stays the source of truth for what is
//! public, and the database is only the editing desk.
//!
//! Routes:
//!
//! | Method | Path                                   | Who      |
//! |--------|----------------------------------------|----------|
//! | GET    | `/admin`, `/admin/app.js`, `/admin/app.css` | anyone (the page asks you to sign in) |
//! | GET    | `/auth/login`, `/auth/callback`        | anyone   |
//! | POST   | `/auth/logout`                         | signed in |
//! | GET    | `/api/me`                              | signed in |
//! | GET    | `/api/articles`, `/api/articles/:slug` | public, published only |
//! | *      | `/api/admin/...`                       | signed in, allowlisted |

mod auth;
mod db;
mod github;
mod handlers;
mod http;

use http::ApiError;
use worker::*;

const ADMIN_HTML: &str = include_str!("../admin/index.html");
const ADMIN_JS: &str = include_str!("../admin/app.js");
const ADMIN_CSS: &str = include_str!("../admin/app.css");

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let response = match route(req, &env).await {
        Ok(response) => response,
        Err(error) => error.into_response()?,
    };
    http::finish(response)
}

async fn route(req: Request, env: &Env) -> std::result::Result<Response, ApiError> {
    let url = req.url()?;
    let path = url.path().to_string();
    let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    let method = req.method();

    match (method.clone(), segments.as_slice()) {
        (Method::Get, []) => http::redirect("/admin"),
        (Method::Get, ["admin"]) => http::admin_page(ADMIN_HTML),
        (Method::Get, ["admin", "app.js"]) => {
            http::asset(ADMIN_JS, "text/javascript; charset=utf-8")
        }
        (Method::Get, ["admin", "app.css"]) => http::asset(ADMIN_CSS, "text/css; charset=utf-8"),

        (Method::Get, ["auth", "login"]) => auth::login(&url, env),
        (Method::Get, ["auth", "callback"]) => auth::callback(&req, &url, env).await,
        (Method::Post, ["auth", "logout"]) => {
            auth::require_same_origin_write(&req, &url)?;
            auth::logout()
        }

        (Method::Get, ["api", "me"]) => {
            let user = auth::require_user(&req, env)?;
            http::json(200, &serde_json::json!({ "login": user.login }))
        }

        (Method::Get, ["api", "articles"]) => handlers::public_list(&url, env).await,
        (Method::Get, ["api", "articles", slug]) => handlers::public_get(slug, env).await,
        (Method::Options, ["api", "articles", ..]) => http::preflight(env),

        (_, ["api", "admin", rest @ ..]) => {
            let user = auth::require_user(&req, env)?;
            if method != Method::Get {
                auth::require_same_origin_write(&req, &url)?;
            }
            let rest: Vec<String> = rest.iter().map(|s| s.to_string()).collect();
            handlers::admin(method, &rest, req, env, &user).await
        }

        _ => Err(ApiError::not_found("No such route.")),
    }
}
