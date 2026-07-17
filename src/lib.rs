pub mod error;
pub mod models;

pub use error::{Error, ErrorKind, Result};
pub use models::{CodeBlock, MenuItem, Page, PageSummary};

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
