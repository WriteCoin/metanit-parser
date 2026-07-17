use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub url: String,
    pub title: String,
    pub content: String,
    pub code_blocks: Vec<CodeBlock>,
    pub menu: Vec<MenuItem>,
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
}

impl Page {
    pub fn word_count(&self) -> usize { self.content.split_whitespace().count() }
    pub fn has_code(&self) -> bool { !self.code_blocks.is_empty() }
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
    pub fn is_section(&self) -> bool { !self.children.is_empty() }
    pub fn is_page(&self) -> bool { self.children.is_empty() }
}
