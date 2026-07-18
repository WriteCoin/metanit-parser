use serde::{Deserialize, Serialize};

/// A parsed tutorial page with title, content, code blocks, and navigation menu.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    /// The URL the page was fetched from.
    pub url: String,
    /// Page title extracted from the `<title>` tag.
    pub title: String,
    /// Plain text content of the `<body>`.
    pub content: String,
    /// Code blocks found on the page, in document order.
    pub code_blocks: Vec<CodeBlock>,
    /// Navigation menu items extracted from links.
    pub menu: Vec<MenuItem>,
}

/// A single code block with optional language annotation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeBlock {
    /// Detected programming language (e.g. `"rust"`, `"python"`), or `None`.
    pub language: Option<String>,
    /// Raw source code text.
    pub code: String,
}

/// An entry in the navigation menu, with optional child items.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItem {
    /// Display title of the menu entry.
    pub title: String,
    /// Target URL.
    pub url: String,
    /// Nested sub-menu items.
    pub children: Vec<MenuItem>,
    /// Whether this item corresponds to the current page.
    pub active: bool,
}

/// Lightweight summary of a [`Page`] — useful for listings and indexes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageSummary {
    /// Page title.
    pub title: String,
    /// Page URL.
    pub url: String,
    /// Number of code blocks on the page.
    pub code_block_count: usize,
    /// Approximate word count of the plain-text content.
    pub word_count: usize,
}

impl Page {
    /// Returns the page title.
    pub fn title(&self) -> &str {
        &self.title
    }
    /// Returns the page URL.
    pub fn url(&self) -> &str {
        &self.url
    }
    /// Returns the plain text content.
    pub fn content(&self) -> &str {
        &self.content
    }
    /// Returns a slice of code blocks.
    pub fn code_blocks(&self) -> &[CodeBlock] {
        &self.code_blocks
    }
    /// Returns a slice of menu items.
    pub fn menu(&self) -> &[MenuItem] {
        &self.menu
    }
    /// Approximate word count based on whitespace splitting.
    pub fn word_count(&self) -> usize {
        self.content.split_whitespace().count()
    }
    /// Whether the page contains at least one code block.
    pub fn has_code(&self) -> bool {
        !self.code_blocks.is_empty()
    }
    /// Creates a lightweight [`PageSummary`] from this page.
    pub fn summary(&self) -> PageSummary {
        PageSummary {
            title: self.title.clone(),
            url: self.url.clone(),
            code_block_count: self.code_blocks.len(),
            word_count: self.word_count(),
        }
    }
}

impl MenuItem {
    /// Whether this item is a section heading with children.
    pub fn is_section(&self) -> bool {
        !self.children.is_empty()
    }
    /// Whether this item is a leaf page (no children).
    pub fn is_page(&self) -> bool {
        self.children.is_empty()
    }
}
