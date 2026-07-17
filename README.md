# metanit-parser

[![CI](https://github.com/example/metanit-parser/actions/workflows/ci.yml/badge.svg)](https://github.com/example/metanit-parser/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A Rust library for parsing tutorial pages from [metanit.com](https://metanit.com).

Extracts structured content — titles, code snippets, navigation menus — from
Metanit's HTML pages.

## Usage

```toml
[dependencies]
metanit-parser = "1.0"
```

```rust
use metanit_parser::{MetanitClient, Page};

let client = MetanitClient::new();
let page = client.fetch_page("https://metanit.com/rust/")?;
println!("Title: {}", page.title());
```

## Features

- Fetch and parse any metanit.com page
- Extract code blocks with language hints
- Parse navigation menu structure
- Caching and retry support

## Documentation

- [Architecture](docs/architecture.md)
- [Git workflow](docs/git-workflow.md)
- [CI/CD pipeline](docs/ci-cd.md)
- [Contributing](CONTRIBUTING.md)

## License

MIT
