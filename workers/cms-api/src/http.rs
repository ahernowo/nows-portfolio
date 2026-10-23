//! Response helpers, the error type, and the security headers every
//! response gets.

use cms_core::FieldError;
use serde::Serialize;
use worker::{Env, Headers, Response};

/// Every handler returns this on failure. It becomes a JSON body:
/// `{"error": {"code": "...", "message": "...", "fields": [...]}}`.
#[derive(Debug)]
pub struct ApiError {
    pub status: u16,
    pub code: &'static str,
    pub message: String,
    pub fields: Vec<FieldError>,
}

impl ApiError {
    pub fn new(status: u16, code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status,
            code,
            message: message.into(),
            fields: Vec::new(),
        }
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(404, "not_found", message)
    }

    pub fn unauthorized() -> Self {
        Self::new(401, "unauthorized", "Sign in to continue.")
    }

    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::new(403, "forbidden", message)
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(400, "bad_request", message)
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        Self::new(409, "conflict", message)
    }

    pub fn invalid(fields: Vec<FieldError>) -> Self {
        Self {
            status: 422,
            code: "invalid",
            message: "Some fields need attention.".into(),
            fields,
        }
    }

    pub fn field(field: &'static str, message: impl Into<String>) -> Self {
        let message = message.into();
        Self {
            status: 422,
            code: "invalid",
            message: message.clone(),
            fields: vec![FieldError { field, message }],
        }
    }

    pub fn internal(detail: impl std::fmt::Display) -> Self {
        // Log the detail for the owner, show a plain message to the browser.
        worker::console_error!("internal error: {detail}");
        Self::new(
            500,
            "internal",
            "Something went wrong on the server. Check the worker logs.",
        )
    }

    pub fn into_response(self) -> worker::Result<Response> {
        #[derive(Serialize)]
        struct Body<'a> {
            error: Inner<'a>,
        }
        #[derive(Serialize)]
        struct Inner<'a> {
            code: &'a str,
            message: &'a str,
            #[serde(skip_serializing_if = "<[FieldError]>::is_empty")]
            fields: &'a [FieldError],
        }
        let body = Body {
            error: Inner {
                code: self.code,
                message: &self.message,
                fields: &self.fields,
            },
        };
        Ok(Response::from_json(&body)?.with_status(self.status))
    }
}

impl From<worker::Error> for ApiError {
    fn from(e: worker::Error) -> Self {
        ApiError::internal(e)
    }
}

pub type ApiResult = Result<Response, ApiError>;

pub fn json<T: Serialize>(status: u16, value: &T) -> ApiResult {
    Ok(Response::from_json(value)?.with_status(status))
}

pub fn redirect(location: &str) -> ApiResult {
    redirect_with_cookies(location, &[])
}

/// A 302 with optional `Set-Cookie` headers. We build it by hand because
/// the platform's `Response.redirect()` has read-only headers.
pub fn redirect_with_cookies(location: &str, cookies: &[String]) -> ApiResult {
    let headers = Headers::new();
    headers.set("Location", location)?;
    for cookie in cookies {
        headers.append("Set-Cookie", cookie)?;
    }
    Ok(Response::empty()?.with_status(302).with_headers(headers))
}

pub fn asset(body: &str, content_type: &str) -> ApiResult {
    let headers = Headers::new();
    headers.set("Content-Type", content_type)?;
    // Assets change only on deploy; a short cache keeps edits visible quickly.
    headers.set("Cache-Control", "public, max-age=300")?;
    Ok(Response::ok(body)?.with_headers(headers))
}

/// Content Security Policy for the admin page: our own script and styles
/// only, no inline code, no framing, API calls to ourselves only.
const ADMIN_CSP: &str = "default-src 'none'; script-src 'self'; style-src 'self'; img-src 'self' https: data:; connect-src 'self'; font-src 'self'; base-uri 'none'; form-action 'self'; frame-ancestors 'none'";

pub fn admin_page(html: &str) -> ApiResult {
    let headers = Headers::new();
    headers.set("Content-Type", "text/html; charset=utf-8")?;
    headers.set("Content-Security-Policy", ADMIN_CSP)?;
    headers.set("Cache-Control", "no-store")?;
    Ok(Response::ok(html)?.with_headers(headers))
}

/// CORS preflight for the public read API.
pub fn preflight(env: &Env) -> ApiResult {
    let headers = Headers::new();
    headers.set("Access-Control-Allow-Origin", &site_origin(env))?;
    headers.set("Access-Control-Allow-Methods", "GET")?;
    headers.set("Access-Control-Max-Age", "86400")?;
    headers.set("Vary", "Origin")?;
    Ok(Response::empty()?.with_status(204).with_headers(headers))
}

pub fn site_origin(env: &Env) -> String {
    env.var("SITE_ORIGIN")
        .map(|v| v.to_string())
        .unwrap_or_default()
}

/// Add the security headers every response should carry, unless a handler
/// already set a more specific value.
pub fn finish(mut response: Response) -> worker::Result<Response> {
    let defaults = [
        ("X-Content-Type-Options", "nosniff"),
        ("Referrer-Policy", "no-referrer"),
        ("X-Frame-Options", "DENY"),
        (
            "Strict-Transport-Security",
            "max-age=31536000; includeSubDomains",
        ),
        (
            "Permissions-Policy",
            "camera=(), microphone=(), geolocation=(), payment=()",
        ),
        ("Cross-Origin-Resource-Policy", "same-site"),
        ("Cache-Control", "no-store"),
    ];
    let headers = response.headers_mut();
    for (name, value) in defaults {
        if headers.get(name)?.is_none() {
            headers.set(name, value)?;
        }
    }
    Ok(response)
}
