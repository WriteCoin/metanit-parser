use scraper::{Html, Selector};
use crate::error::{Error, ErrorKind, Result};
use crate::models::{CodeBlock, MenuItem, Page};

fn parse_title(document: &Html) -> String {
    Selector::parse("title").ok()
        .and_then(|sel| document.select(&sel).next())
        .map(|el| el.text().collect::<String>().trim().to_string())
        .unwrap_or_default()
}

fn parse_code_blocks(document: &Html) -> Vec<CodeBlock> {
    let selector = Selector::parse("pre code").expect("code selector");
    document.select(&selector).map(|el| {
        let code = el.text().collect::<String>();
        let language = el.value().attr("class")
            .and_then(|c| c.split_whitespace()
                .find(|cls| cls.starts_with("language-"))
                .map(|cls| cls.split_once('-').map_or(cls, |(_, l)| l)))
            .map(String::from);
        CodeBlock { language, code }
    }).collect()
}

fn resolve_base_url(url: &str) -> String {
    if let Ok(parsed) = url::Url::parse(url) {
        return parsed.origin().ascii_serialization();
    }
    url.to_string()
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
        code_blocks: parse_code_blocks(&document),
        menu: Vec::new(),
    })
}
