//! `metanit-parser` — A Rust library for parsing [metanit.com](https://metanit.com) tutorial pages.
//!
//! # Quick start
//!
//! ```rust,no_run
//! use metanit_parser::MetanitClient;
//!
//! let client = MetanitClient::new();
//! match client.fetch_page("https://metanit.com/rust/") {
//!     Ok(page) => println!("Title: {}", page.title()),
//!     Err(e) => eprintln!("Error: {}", e),
//! }
//! ```
//!
//! # Architecture
//!
//! - [`MetanitClient`] — blocking HTTP client with built-in caching and retry
//! - [`parse_page`] — low-level HTML parser (no network needed)
//! - [`Page`], [`CodeBlock`], [`MenuItem`] — structured data models

pub mod client;
pub mod error;
pub mod models;
pub mod parser;

pub use client::MetanitClient;
pub use error::{Error, ErrorKind, Result};
pub use models::{CodeBlock, MenuItem, Page, PageSummary};
pub use parser::{code_block_count_by_language, extract_code_languages, parse_page};

/// Returns the current library version.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod lib_tests {
    use super::*;
    #[test]
    fn test_version() {
        assert!(!version().is_empty());
    }
    #[test]
    fn test_client_default() {
        MetanitClient::new().clear_cache();
    }
}
