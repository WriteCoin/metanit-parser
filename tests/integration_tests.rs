use metanit_parser::models::{CodeBlock, Page};
use metanit_parser::parser;
use metanit_parser::MetanitClient;
use std::time::Duration;

mod mock_tests {
    use metanit_parser::MetanitClient;
    use std::time::Duration;

    const FIXTURE: &str = r#"<html><head><title>Rust</title></head><body><p>Hello</p></body></html>"#;

    #[test]
    fn test_mock_successful_parse() {
        let mut server = mockito::Server::new();
        let url = server.url();
        server.mock("GET", "/").with_status(200).with_body(FIXTURE).create();
        let client = MetanitClient::new().with_timeout(Duration::from_secs(10));
        let page = client.fetch_page(&format!("{}/", url)).unwrap();
        assert_eq!(page.title(), "Rust");
    }

    #[test]
    fn test_mock_http_error() {
        let mut server = mockito::Server::new();
        let url = server.url();
        server.mock("GET", "/").with_status(404).create();
        let client = MetanitClient::new()
            .with_max_retries(0)
            .with_timeout(Duration::from_secs(10));
        let result = client.fetch_page(&format!("{}/", url));
        assert!(result.is_err());
    }

    #[test]
    fn test_mock_cache_hit() {
        let mut server = mockito::Server::new();
        let url = server.url();
        let mock = server.mock("GET", "/").with_status(200).with_body(FIXTURE).expect_at_most(1).create();
        let client = MetanitClient::new()
            .with_cache_ttl(Duration::from_secs(60))
            .with_timeout(Duration::from_secs(10));
        let _ = client.fetch_page(&format!("{}/", url));
        let _ = client.fetch_page(&format!("{}/", url));
        mock.assert();
    }

    #[test]
    fn test_mock_clear_cache() {
        let mut server = mockito::Server::new();
        let url = server.url();
        let mock = server.mock("GET", "/").with_status(200).with_body(FIXTURE).expect_at_least(2).create();
        let client = MetanitClient::new()
            .with_cache_ttl(Duration::from_secs(3600))
            .with_timeout(Duration::from_secs(10));
        let _ = client.fetch_page(&format!("{}/", url));
        client.clear_cache();
        let _ = client.fetch_page(&format!("{}/", url));
        mock.assert();
    }
}

const MOCK_HTML: &str = r#"
<!DOCTYPE html>
<html><head><title>Metanit Rust Tutorial</title></head>
<body>
    <nav><a href="https://metanit.com/rust/">Home</a><a href="https://metanit.com/rust/2.1.php">Variables</a></nav>
    <h1>Introduction to Rust</h1>
    <p>Rust is a systems language focused on safety.</p>
    <pre><code class="language-rust">fn main() { println!("Hello!"); }</code></pre>
    <pre><code>let x = 42;</code></pre>
</body></html>
"#;

#[test]
fn test_full_page_parse() {
    let page = parser::parse_page("https://metanit.com/rust/", MOCK_HTML).unwrap();
    assert_eq!(page.title(), "Metanit Rust Tutorial");
    assert!(page.has_code());
    assert_eq!(page.code_blocks().len(), 2);
    assert!(page.word_count() > 5);
}

#[test]
fn test_code_blocks_with_mixed_language() {
    let page = parser::parse_page("https://metanit.com/rust/", MOCK_HTML).unwrap();
    let blocks = page.code_blocks();
    assert_eq!(blocks[0].language.as_deref(), Some("rust"));
    assert_eq!(blocks[1].language, None);
}

#[test]
fn test_serialization_roundtrip() {
    let page = Page {
        url: "https://metanit.com/".into(),
        title: "Test".into(),
        content: "Content".into(),
        code_blocks: vec![CodeBlock {
            language: Some("rust".into()),
            code: "fn m() {}".into(),
        }],
        menu: vec![],
    };
    let json = serde_json::to_string(&page).unwrap();
    let back: Page = serde_json::from_str(&json).unwrap();
    assert_eq!(back.title(), "Test");
    assert_eq!(back.code_blocks().len(), 1);
}

#[test]
fn test_empty_html_fails() {
    assert!(parser::parse_page("https://x.com", "<html></html>").is_err());
}

#[test]
fn test_client_config() {
    let c = MetanitClient::new()
        .with_timeout(Duration::from_secs(10))
        .with_cache_ttl(Duration::from_secs(30))
        .with_max_retries(1);
    c.clear_cache();
}

#[test]
fn test_page_summary() {
    let p = Page {
        url: "https://metanit.com/rust/2.1.php".into(),
        title: "Variables".into(),
        content: "Variables in Rust are immutable by default.".into(),
        code_blocks: vec![
            CodeBlock {
                language: Some("rust".into()),
                code: "let x = 5;".into(),
            },
            CodeBlock {
                language: Some("rust".into()),
                code: "let mut y = 10;".into(),
            },
        ],
        menu: vec![],
    };
    let s = p.summary();
    assert_eq!(s.title, "Variables");
    assert_eq!(s.code_block_count, 2);
    assert!(s.word_count > 0);
}
