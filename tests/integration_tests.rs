use metanit_parser::parser;
use metanit_parser::models::{CodeBlock, Page};

const MOCK_HTML: &str = r#"
<!DOCTYPE html><html><head><title>Metanit Tutorial</title></head>
<body><h1>Intro</h1><pre><code class="language-rust">fn main() {}</code></pre></body></html>
"#;

#[test]
fn test_parse_realistic_page() {
    let page = parser::parse_page("https://metanit.com/rust/", MOCK_HTML).unwrap();
    assert_eq!(page.title(), "Metanit Tutorial");
    assert!(page.has_code());
}

#[test]
fn test_page_without_code_blocks() {
    let html = r#"<html><head><title>Empty</title></head><body><p>Text.</p></body></html>"#;
    let page = parser::parse_page("https://metanit.com/", html).unwrap();
    assert!(!page.has_code());
}

#[test]
fn test_serialization_roundtrip() {
    let page = Page {
        url: "https://metanit.com/".into(), title: "Rust".into(),
        content: "text".into(),
        code_blocks: vec![CodeBlock { language: Some("rust".into()), code: "fn m() {}".into() }],
        menu: vec![],
    };
    let json = serde_json::to_string(&page).unwrap();
    let back: Page = serde_json::from_str(&json).unwrap();
    assert_eq!(back.title, "Rust");
}
