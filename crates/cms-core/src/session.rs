//! Signed session tokens for the admin cookie.
//!
//! Format: `base64url(json claims) "." base64url(HMAC-SHA256(claims part))`.
//! The server keeps no session table: the signature proves the token came
//! from us, and `exp` limits how long a stolen cookie is useful.

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Keys shorter than this are rejected, so a weak secret fails loudly at
/// deploy time instead of quietly weakening every session.
pub const MIN_KEY_BYTES: usize = 32;

/// Admin sessions last 8 hours.
pub const SESSION_SECONDS: u64 = 8 * 60 * 60;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Claims {
    /// GitHub login of the signed-in admin.
    pub sub: String,
    /// Issued at, Unix seconds.
    pub iat: u64,
    /// Expires at, Unix seconds.
    pub exp: u64,
}

impl Claims {
    pub fn new(login: &str, now: u64) -> Self {
        Self {
            sub: login.to_string(),
            iat: now,
            exp: now + SESSION_SECONDS,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionError {
    KeyTooShort,
    Malformed,
    BadSignature,
    Expired,
}

impl std::fmt::Display for SessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            SessionError::KeyTooShort => "session key must be at least 32 bytes",
            SessionError::Malformed => "session token is malformed",
            SessionError::BadSignature => "session signature does not match",
            SessionError::Expired => "session has expired",
        };
        f.write_str(msg)
    }
}

impl std::error::Error for SessionError {}

fn mac(key: &[u8]) -> Result<HmacSha256, SessionError> {
    if key.len() < MIN_KEY_BYTES {
        return Err(SessionError::KeyTooShort);
    }
    HmacSha256::new_from_slice(key).map_err(|_| SessionError::KeyTooShort)
}

/// Sign claims into a token.
pub fn sign(claims: &Claims, key: &[u8]) -> Result<String, SessionError> {
    let json = serde_json::to_vec(claims).map_err(|_| SessionError::Malformed)?;
    let payload = URL_SAFE_NO_PAD.encode(json);
    let mut m = mac(key)?;
    m.update(payload.as_bytes());
    let sig = URL_SAFE_NO_PAD.encode(m.finalize().into_bytes());
    Ok(format!("{payload}.{sig}"))
}

/// Check a token's signature and expiry, and return its claims.
pub fn verify(token: &str, key: &[u8], now: u64) -> Result<Claims, SessionError> {
    let (payload, sig) = token.split_once('.').ok_or(SessionError::Malformed)?;
    let sig = URL_SAFE_NO_PAD
        .decode(sig)
        .map_err(|_| SessionError::Malformed)?;

    let mut m = mac(key)?;
    m.update(payload.as_bytes());
    // `verify_slice` compares in constant time, so timing reveals nothing.
    m.verify_slice(&sig)
        .map_err(|_| SessionError::BadSignature)?;

    let json = URL_SAFE_NO_PAD
        .decode(payload)
        .map_err(|_| SessionError::Malformed)?;
    let claims: Claims = serde_json::from_slice(&json).map_err(|_| SessionError::Malformed)?;
    if now >= claims.exp {
        return Err(SessionError::Expired);
    }
    Ok(claims)
}

/// Decode a session key given as base64 (standard or URL-safe) or as raw text.
pub fn decode_key(secret: &str) -> Vec<u8> {
    let s = secret.trim();
    base64::engine::general_purpose::STANDARD
        .decode(s)
        .or_else(|_| URL_SAFE_NO_PAD.decode(s))
        .unwrap_or_else(|_| s.as_bytes().to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    const KEY: &[u8] = b"0123456789abcdef0123456789abcdef";

    #[test]
    fn round_trip() {
        let claims = Claims::new("nows", 1_000);
        let token = sign(&claims, KEY).expect("sign");
        assert_eq!(verify(&token, KEY, 1_001), Ok(claims));
    }

    #[test]
    fn expired() {
        let token = sign(&Claims::new("nows", 1_000), KEY).expect("sign");
        assert_eq!(
            verify(&token, KEY, 1_000 + SESSION_SECONDS),
            Err(SessionError::Expired)
        );
    }

    #[test]
    fn wrong_key() {
        let token = sign(&Claims::new("nows", 1_000), KEY).expect("sign");
        let other = b"ffffffffffffffffffffffffffffffff";
        assert_eq!(
            verify(&token, other, 1_001),
            Err(SessionError::BadSignature)
        );
    }

    #[test]
    fn tampered_payload() {
        let token = sign(&Claims::new("nows", 1_000), KEY).expect("sign");
        let (_, sig) = token.split_once('.').expect("dot");
        let forged = URL_SAFE_NO_PAD.encode(br#"{"sub":"attacker","iat":1000,"exp":99999999999}"#);
        assert_eq!(
            verify(&format!("{forged}.{sig}"), KEY, 1_001),
            Err(SessionError::BadSignature)
        );
    }

    #[test]
    fn short_key_rejected() {
        assert_eq!(
            sign(&Claims::new("nows", 1), b"short"),
            Err(SessionError::KeyTooShort)
        );
    }

    #[test]
    fn garbage() {
        assert_eq!(verify("nodot", KEY, 0), Err(SessionError::Malformed));
        assert_eq!(verify("a.!!!", KEY, 0), Err(SessionError::Malformed));
    }

    #[test]
    fn key_decoding() {
        let b64 = base64::engine::general_purpose::STANDARD.encode(KEY);
        assert_eq!(decode_key(&b64), KEY);
        assert_eq!(decode_key("not base64 !!"), b"not base64 !!");
    }
}
