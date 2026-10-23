//! Sign-in with GitHub, the session cookie, and request checks.
//!
//! GitHub only tells us who you are. Whether you may edit is decided by
//! `ADMIN_LOGINS`, which is checked on every request, so removing a name
//! from it signs that person out at once.

use cms_core::oauth::{self, CookieOptions};
use cms_core::session::{self, Claims};
use serde::Deserialize;
use worker::{Env, Fetch, Headers, Method, Request, RequestInit, Url};

use crate::http::{redirect_with_cookies, ApiError, ApiResult};

const STATE_COOKIE: &str = "cms_oauth_state";
pub const SESSION_COOKIE: &str = "cms_session";
const USER_AGENT: &str = "nows-portfolio-cms";

pub struct User {
    pub login: String,
}

struct OAuthConfig {
    client_id: String,
    client_secret: String,
}

fn secret(env: &Env, name: &str) -> Result<String, ApiError> {
    env.secret(name)
        .map(|s| s.to_string())
        .or_else(|_| env.var(name).map(|v| v.to_string()))
        .map_err(|_| {
            ApiError::internal(format!(
                "{name} is not set. Run `npx wrangler secret put {name}`."
            ))
        })
}

fn oauth_config(env: &Env) -> Result<OAuthConfig, ApiError> {
    Ok(OAuthConfig {
        client_id: secret(env, "GITHUB_CLIENT_ID")?,
        client_secret: secret(env, "GITHUB_CLIENT_SECRET")?,
    })
}

fn session_key(env: &Env) -> Result<Vec<u8>, ApiError> {
    let key = session::decode_key(&secret(env, "SESSION_KEY")?);
    if key.len() < session::MIN_KEY_BYTES {
        return Err(ApiError::internal(
            "SESSION_KEY must be at least 32 bytes. See the README for how to make one.",
        ));
    }
    Ok(key)
}

fn admin_logins(env: &Env) -> String {
    env.var("ADMIN_LOGINS")
        .map(|v| v.to_string())
        .unwrap_or_default()
}

pub fn now_secs() -> u64 {
    worker::Date::now().as_millis() / 1000
}

fn callback_url(url: &Url) -> String {
    format!("{}/auth/callback", url.origin().ascii_serialization())
}

pub fn random_hex(len: usize) -> Result<String, ApiError> {
    let mut buf = vec![0u8; len];
    getrandom::getrandom(&mut buf).map_err(ApiError::internal)?;
    Ok(oauth::to_hex(&buf))
}

/// `GET /auth/login`: send the browser to GitHub with a random state.
pub fn login(url: &Url, env: &Env) -> ApiResult {
    let config = oauth_config(env)?;
    let state = random_hex(16)?;
    // No scope: we only need to read the public profile to learn the login.
    let location = oauth::authorize_url(&config.client_id, &callback_url(url), "", &state);
    let cookie = oauth::set_cookie(
        STATE_COOKIE,
        &state,
        &CookieOptions {
            path: "/auth",
            max_age: 600,
            same_site: "Lax",
        },
    );
    redirect_with_cookies(&location, &[cookie])
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: Option<String>,
    error: Option<String>,
}

#[derive(Deserialize)]
struct GitHubUser {
    login: String,
}

/// `GET /auth/callback`: finish sign-in and set the session cookie.
pub async fn callback(req: &Request, url: &Url, env: &Env) -> ApiResult {
    let fail = |reason: &str| {
        redirect_with_cookies(
            &format!("/admin?signin={reason}"),
            &[oauth::clear_cookie(STATE_COOKIE, "/auth")],
        )
    };

    let query: std::collections::HashMap<String, String> = url.query_pairs().into_owned().collect();
    if query.contains_key("error") {
        return fail("cancelled");
    }
    let (Some(code), Some(state)) = (query.get("code"), query.get("state")) else {
        return fail("failed");
    };
    let cookies = req.headers().get("Cookie")?.unwrap_or_default();
    let expected = oauth::cookie(&cookies, STATE_COOKIE).unwrap_or_default();
    if expected.is_empty() || !oauth::constant_time_eq(expected, state) {
        return fail("expired");
    }

    let config = oauth_config(env)?;
    let token = match exchange_code(&config, code, &callback_url(url)).await {
        Ok(t) => t,
        Err(e) => {
            worker::console_error!("token exchange failed: {e}");
            return fail("failed");
        }
    };
    let login = match fetch_login(&token).await {
        Ok(l) => l,
        Err(e) => {
            worker::console_error!("could not read GitHub user: {e}");
            return fail("failed");
        }
    };
    // The GitHub token is dropped here. We never store it.

    if !oauth::is_allowed_login(&login, &admin_logins(env)) {
        worker::console_warn!("sign-in refused for {login}");
        return fail("not-allowed");
    }

    let token = session::sign(&Claims::new(&login, now_secs()), &session_key(env)?)
        .map_err(ApiError::internal)?;
    let session_cookie = oauth::set_cookie(
        SESSION_COOKIE,
        &token,
        &CookieOptions {
            path: "/",
            max_age: session::SESSION_SECONDS,
            same_site: "Strict",
        },
    );
    redirect_with_cookies(
        "/admin",
        &[session_cookie, oauth::clear_cookie(STATE_COOKIE, "/auth")],
    )
}

async fn exchange_code(
    config: &OAuthConfig,
    code: &str,
    redirect_uri: &str,
) -> Result<String, String> {
    let headers = Headers::new();
    for (k, v) in [
        ("Accept", "application/json"),
        ("Content-Type", "application/x-www-form-urlencoded"),
        ("User-Agent", USER_AGENT),
    ] {
        headers.set(k, v).map_err(|e| e.to_string())?;
    }
    let body =
        oauth::token_request_body(&config.client_id, &config.client_secret, code, redirect_uri);
    let mut init = RequestInit::new();
    init.with_method(Method::Post)
        .with_headers(headers)
        .with_body(Some(worker::wasm_bindgen::JsValue::from_str(&body)));
    let request = Request::new_with_init(oauth::GITHUB_TOKEN, &init).map_err(|e| e.to_string())?;
    let mut response = Fetch::Request(request)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let parsed: TokenResponse = response.json().await.map_err(|e| e.to_string())?;
    match parsed.access_token {
        Some(t) if !t.is_empty() => Ok(t),
        _ => Err(parsed.error.unwrap_or_else(|| "no access_token".into())),
    }
}

async fn fetch_login(token: &str) -> Result<String, String> {
    let headers = Headers::new();
    for (k, v) in [
        ("Accept", "application/vnd.github+json"),
        ("User-Agent", USER_AGENT),
        ("X-GitHub-Api-Version", "2022-11-28"),
    ] {
        headers.set(k, v).map_err(|e| e.to_string())?;
    }
    headers
        .set("Authorization", &format!("Bearer {token}"))
        .map_err(|e| e.to_string())?;
    let mut init = RequestInit::new();
    init.with_method(Method::Get).with_headers(headers);
    let request = Request::new_with_init(&format!("{}/user", oauth::GITHUB_API), &init)
        .map_err(|e| e.to_string())?;
    let mut response = Fetch::Request(request)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if response.status_code() != 200 {
        return Err(format!("GitHub answered {}", response.status_code()));
    }
    let user: GitHubUser = response.json().await.map_err(|e| e.to_string())?;
    Ok(user.login)
}

/// `POST /auth/logout`: forget the session cookie.
pub fn logout() -> ApiResult {
    let headers = Headers::new();
    headers.set("Set-Cookie", &oauth::clear_cookie(SESSION_COOKIE, "/"))?;
    Ok(worker::Response::empty()?
        .with_status(204)
        .with_headers(headers))
}

/// Who is making this request? Fails with 401 when the session cookie is
/// missing, forged, expired, or belongs to someone no longer allowlisted.
pub fn require_user(req: &Request, env: &Env) -> Result<User, ApiError> {
    let cookies = req.headers().get("Cookie")?.unwrap_or_default();
    let token = oauth::cookie(&cookies, SESSION_COOKIE).ok_or_else(ApiError::unauthorized)?;
    let claims = session::verify(token, &session_key(env)?, now_secs())
        .map_err(|_| ApiError::unauthorized())?;
    if !oauth::is_allowed_login(&claims.sub, &admin_logins(env)) {
        return Err(ApiError::forbidden(
            "This GitHub account is not allowed to edit.",
        ));
    }
    Ok(User { login: claims.sub })
}

/// Requests that change data must come from our own admin page.
///
/// Three independent checks, any one of which stops a cross-site attack:
/// the cookie is `SameSite=Strict`; the page must send `X-CMS-Request: 1`,
/// a custom header other sites cannot send without a CORS preflight we never
/// approve; and when the browser says where the request came from
/// (`Sec-Fetch-Site`, `Origin`), it must be us.
pub fn require_same_origin_write(req: &Request, url: &Url) -> Result<(), ApiError> {
    let headers = req.headers();
    if headers.get("X-CMS-Request")?.as_deref() != Some("1") {
        return Err(ApiError::forbidden(
            "Missing X-CMS-Request header. Use the admin page.",
        ));
    }
    if let Some(site) = headers.get("Sec-Fetch-Site")? {
        if site != "same-origin" {
            return Err(ApiError::forbidden("Cross-site request refused."));
        }
    }
    if let Some(origin) = headers.get("Origin")? {
        if origin != url.origin().ascii_serialization() {
            return Err(ApiError::forbidden("Request from another origin refused."));
        }
    }
    Ok(())
}
