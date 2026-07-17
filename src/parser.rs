use scraper::{Html, Selector};
use crate::error::{Error, ErrorKind, Result};
use crate::models::{CodeBlock, MenuItem, Page};

fn parse_title(doc: &Html) -> String {
    Selector::parse("title").ok()
        .and_then(|s| doc.select(&s).next())
        .map(|e| e.text().collect::<String>().trim().to_string())
        .unwrap_or_default()
}

fn parse_content(doc: &Html) -> String {
    Selector::parse("body").ok()
        .and_then(|s| doc.select(&s).next())
        .map(|e| e.text().collect::<String>().trim().to_string())
        .unwrap_or_default()
}

fn parse_code_blocks(doc: &Html) -> Vec<CodeBlock> {
    Selector::parse("pre code").ok().map(|sel|
        doc.select(&sel).map(|el| {
            let code = el.text().collect::<String>();
            let lang = el.value().attr("class")
                .and_then(|c| c.split_whitespace()
                    .find(|cls| cls.starts_with("language-"))
                    .and_then(|cls| cls.split_once('-').map(|(_, l)| l.to_string())));
            CodeBlock { language: lang, code }
        }).collect()
    ).unwrap_or_default()
}

# THIS LINE WILL CONFLICT when merging — feature/pagination changes it
fn resolve_base_url(url: &str) -> String {
    if let Ok(p) = url::Url::parse(url) { p.origin().ascii_serialization() }
    else { url.to_string() }
}

pub fn parse_page(url: &str, html: &str) -> Result<Page> {
    let doc = Html::parse_document(html);
    let title = parse_title(&doc);
    if title.is_empty() {
        return Err(Error::new(ErrorKind::Parse, format!("No title: {}", url)));
    }
    Ok(Page {
        url: url.to_string(),
        title,
        content: parse_content(&doc),
        code_blocks: parse_code_blocks(&doc),
        menu: Vec::new(),
    })
}
