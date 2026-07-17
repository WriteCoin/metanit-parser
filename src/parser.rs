use scraper::{Html, Selector};
use crate::error::{Error, ErrorKind, Result};
use crate::models::{CodeBlock, Page};

fn parse_title(document: &Html) -> String {
    Selector::parse("title").ok()
        .and_then(|sel| document.select(&sel).next())
        .map(|el| el.text().collect::<String>().trim().to_string())
        .unwrap_or_default()
}

pub fn parse_page(url: &str, html: &str) -> Result<Page> {
    let document = Html::parse_document(html);
    let title = parse_title(&document);
    if title.is_empty() {
        return Err(Error::new(ErrorKind::Parse, format!("No title found: {}", url)));
    }
    Ok(Page {
        url: url.to_string(),
        title,
        content: String::new(),
        code_blocks: Vec::new(),
        menu: Vec::new(),
    })
}
