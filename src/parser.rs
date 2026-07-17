use scraper::{Html, Selector};
use crate::error::{Error, ErrorKind, Result};
use crate::models::{CodeBlock, MenuItem, Page};

fn parse_title(doc: &Html) -> String {
    Selector::parse("title").ok()
        .and_then(|s| doc.select(&s).next())
        .map(|e| e.text().collect::<String>().trim().to_string())
        .unwrap_or_default()
}

fn parse_menu(doc: &Html, base_url: &str) -> Vec<MenuItem> {
    Selector::parse("a[href]").ok().map(|sel|
        doc.select(&sel).filter_map(|el| {
            let href = el.value().attr("href")?;
            if !href.contains("metanit.com") || href.contains('#') { return None; }
            let title = el.text().collect::<String>().trim().to_string();
            if title.is_empty() { return None; }
            let url = if href.starts_with("http") { href.to_string() }
                      else { format!("{}/{}", base_url.trim_end_matches('/'), href.trim_start_matches('/')) };
            Some(MenuItem { title, url, children: Vec::new(), active: false })
        }).collect()
    ).unwrap_or_default()
}

fn resolve_base_url(url: &str) -> String {
    url::Url::parse(url).map(|p| p.origin().ascii_serialization()).unwrap_or_else(|_| url.to_string())
}

pub fn parse_page(url: &str, html: &str) -> Result<Page> {
    let doc = Html::parse_document(html);
    let title = parse_title(&doc);
    if title.is_empty() {
        return Err(Error::new(ErrorKind::Parse, format!("No title: {}", url)));
    }
    let base = resolve_base_url(url);
    Ok(Page {
        url: url.to_string(),
        title,
        content: String::new(),
        code_blocks: Vec::new(),
        menu: parse_menu(&doc, &base),
    })
}
