use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub url: String,
    pub title: String,
    pub content: String,
    pub content_html: String,
    pub code_blocks: Vec<CodeBlock>,
    pub menu: Vec<MenuItem>,
    pub total_pages: Option<usize>,
    pub current_page: Option<usize>,
    pub prev_url: Option<String>,
    pub next_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeBlock {
    pub language: Option<String>,
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItem {
    pub title: String,
    pub url: String,
    pub children: Vec<MenuItem>,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageSummary {
    pub title: String,
    pub url: String,
    pub code_block_count: usize,
    pub word_count: usize,
    pub total_pages: Option<usize>,
    pub current_page: Option<usize>,
    pub prev_url: Option<String>,
    pub next_url: Option<String>,
}

impl Page {
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn url(&self) -> &str {
        &self.url
    }
    pub fn content(&self) -> &str {
        &self.content
    }
    pub fn code_blocks(&self) -> &[CodeBlock] {
        &self.code_blocks
    }
    pub fn menu(&self) -> &[MenuItem] {
        &self.menu
    }
    pub fn word_count(&self) -> usize {
        self.content.split_whitespace().count()
    }
    pub fn has_code(&self) -> bool {
        !self.code_blocks.is_empty()
    }
    pub fn summary(&self) -> PageSummary {
        PageSummary {
            title: self.title.clone(),
            url: self.url.clone(),
            code_block_count: self.code_blocks.len(),
            word_count: self.word_count(),
            total_pages: self.total_pages,
            current_page: self.current_page,
            prev_url: self.prev_url.clone(),
            next_url: self.next_url.clone(),
        }
    }
}

impl MenuItem {
    pub fn is_section(&self) -> bool {
        !self.children.is_empty()
    }
    pub fn is_page(&self) -> bool {
        self.children.is_empty()
    }
}
