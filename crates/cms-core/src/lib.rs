//! Shared, pure logic for the portfolio CMS.
//!
//! Nothing in this crate touches the network, the clock or the file system.
//! Callers pass in the current time and any secrets. That keeps every function
//! easy to test with `cargo test`, and lets the same code run natively (the
//! CLI tools) and as WebAssembly (the Cloudflare Workers).

pub mod article;
pub mod date;
pub mod frontmatter;
pub mod oauth;
pub mod preview;
pub mod session;
pub mod slug;

pub use article::{validate, ArticleInput, FieldError, Status, ValidArticle};
pub use date::Date;
