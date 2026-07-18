use crate::error::{Error, ErrorKind, Result};
use crate::models::{CodeBlock, MenuItem, Page};
use scraper::{Html, Selector};
use std::collections::HashMap;

fn parse_title(doc: &Html) -> String {
    Selector::parse("title")
        .ok()
        .and_then(|s| doc.select(&s).next())
        .map(|e| e.text().collect::<String>().trim().to_string())
        .unwrap_or_default()
}

fn parse_body_raw(doc: &Html, extract: fn(&scraper::ElementRef) -> String) -> String {
    Selector::parse("body")
        .ok()
        .and_then(|s| doc.select(&s).next())
        .map(|e| extract(&e))
        .unwrap_or_default()
}

fn parse_content(doc: &Html) -> String {
    parse_body_raw(doc, |e| e.text().collect::<String>().trim().to_string())
}

fn parse_content_html(doc: &Html) -> String {
    parse_body_raw(doc, |e| e.inner_html())
}

fn parse_code_blocks(doc: &Html) -> Vec<CodeBlock> {
    Selector::parse("pre code")
        .ok()
        .map(|sel| {
            doc.select(&sel)
                .map(|el| {
                    let code = el.text().collect::<String>();
                    let lang = el.value().attr("class").and_then(|c| {
                        c.split_whitespace()
                            .find(|cls| cls.starts_with("language-") || cls.starts_with("lang-"))
                            .and_then(|cls| cls.split_once('-').map(|(_, l)| l.to_string()))
                    });
                    CodeBlock {
                        language: lang,
                        code,
                    }
                })
                .collect()
        })
        .unwrap_or_default()
}

fn parse_menu(doc: &Html, base_url: &str) -> Vec<MenuItem> {
    Selector::parse("a[href]")
        .ok()
        .map(|sel| {
            doc.select(&sel)
                .filter_map(|el| {
                    let href = el.value().attr("href")?;
                    if !href.contains("metanit.com") || href.contains('#') {
                        return None;
                    }
                    let title = el.text().collect::<String>().trim().to_string();
                    if title.is_empty() {
                        return None;
                    }
                    let url = if href.starts_with("http") {
                        href.to_string()
                    } else {
                        format!(
                            "{}/{}",
                            base_url.trim_end_matches('/'),
                            href.trim_start_matches('/')
                        )
                    };
                    Some(MenuItem {
                        title,
                        url,
                        children: Vec::new(),
                        active: false,
                    })
                })
                .collect()
        })
        .unwrap_or_default()
}

fn resolve_base_url(url: &str) -> String {
    url::Url::parse(url)
        .map(|p| p.origin().ascii_serialization())
        .unwrap_or_else(|_| url.to_string())
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
        content: parse_content(&doc),
        content_html: parse_content_html(&doc),
        code_blocks: parse_code_blocks(&doc),
        menu: parse_menu(&doc, &base),
    })
}

pub fn extract_code_languages(page: &Page) -> Vec<&str> {
    page.code_blocks
        .iter()
        .filter_map(|cb| cb.language.as_deref())
        .collect()
}

pub fn code_block_count_by_language(page: &Page) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for cb in &page.code_blocks {
        let lang = cb.language.clone().unwrap_or_else(|| "unknown".to_string());
        *counts.entry(lang).or_insert(0) += 1;
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> &'static str {
        r#"<html><head><title>Rust</title></head><body><pre><code class="language-rust">fn main() {}</code></pre></body></html>"#
    }

    #[test]
    fn test_parse_title() {
        let d = Html::parse_document(sample());
        assert_eq!(parse_title(&d), "Rust");
    }

    #[test]
    fn test_parse_code_blocks() {
        let d = Html::parse_document(sample());
        let blocks = parse_code_blocks(&d);
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].language.as_deref(), Some("rust"));
    }

    #[test]
    fn test_extract_languages() {
        let p = Page {
            url: "".into(),
            title: "".into(),
            content: "".into(),
            code_blocks: vec![CodeBlock {
                language: Some("rust".into()),
                code: "a".into(),
            }],
            menu: vec![],
            content_html: "".into(),
        };
        assert_eq!(extract_code_languages(&p), vec!["rust"]);
    }

    #[test]
    fn test_count_by_language() {
        let p = Page {
            url: "".into(),
            title: "".into(),
            content: "".into(),
            code_blocks: vec![
                CodeBlock {
                    language: Some("rust".into()),
                    code: "a".into(),
                },
                CodeBlock {
                    language: Some("rust".into()),
                    code: "b".into(),
                },
                CodeBlock {
                    language: Some("python".into()),
                    code: "c".into(),
                },
            ],
            menu: vec![],
            content_html: "".into(),
        };
        let c = code_block_count_by_language(&p);
        assert_eq!(c.get("rust"), Some(&2));
        assert_eq!(c.get("python"), Some(&1));
    }

    #[test]
    fn test_empty_parse_fails() {
        assert!(parse_page("https://x.com", "<html></html>").is_err());
    }

    #[test]
    fn test_page_has_code() {
        let w = Page {
            url: "".into(),
            title: "".into(),
            content: "".into(),
            code_blocks: vec![CodeBlock {
                language: None,
                code: "x".into(),
            }],
            menu: vec![],
            content_html: "".into(),
        };
        assert!(w.has_code());
        let wo = Page {
            url: "".into(),
            title: "".into(),
            content: "".into(),
            code_blocks: vec![],
            menu: vec![],
            content_html: "".into(),
        };
        assert!(!wo.has_code());
    }

    #[test]
    fn test_page_word_count() {
        let p = Page {
            url: "".into(),
            title: "".into(),
            content: " a b  c ".into(),
            code_blocks: vec![],
            menu: vec![],
            content_html: "".into(),
        };
        assert_eq!(p.word_count(), 3);
    }

    #[test]
    fn test_menu_classification() {
        let s = MenuItem {
            title: "S".into(),
            url: "/".into(),
            children: vec![MenuItem {
                title: "C".into(),
                url: "/c".into(),
                children: vec![],
                active: false,
            }],
            active: false,
        };
        assert!(s.is_section());
        assert!(!s.is_page());
        let p = MenuItem {
            title: "P".into(),
            url: "/p".into(),
            children: vec![],
            active: false,
        };
        assert!(p.is_page());
    }

    #[test]
    fn test_page_summary() {
        let p = Page {
            url: "".into(),
            title: "T".into(),
            content: "one two".into(),
            code_blocks: vec![CodeBlock {
                language: None,
                code: "x".into(),
            }],
            menu: vec![],
            content_html: "".into(),
        };
        let s = p.summary();
        assert_eq!(s.title, "T");
        assert_eq!(s.code_block_count, 1);
        assert_eq!(s.word_count, 2);
    }

    #[test]
    fn test_resolve_base() {
        assert_eq!(
            resolve_base_url("https://metanit.com/rust/"),
            "https://metanit.com"
        );
    }
}
