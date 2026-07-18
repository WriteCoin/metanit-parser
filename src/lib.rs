pub mod client;
pub mod error;
pub mod models;
pub mod parser;

pub use client::MetanitClient;
pub use error::{Error, ErrorKind, Result};
pub use models::{CodeBlock, MenuItem, Page, PageSummary};
pub use parser::{code_block_count_by_language, extract_code_languages, parse_page};

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
