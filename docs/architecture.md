# Architecture

## Overview

```
┌─────────────┐     ┌──────────────┐     ┌────────────┐
│ MetanitClient │────▶│   Parser     │────▶│   Page     │
└─────────────┘     └──────────────┘     └────────────┘
       │                                       │
       ▼                                       ▼
   reqwest                               scraper crate
```

## Modules

- **`client`** — HTTP transport. Responsible for fetching HTML, caching
  responses, and retrying on transient failures.
- **`parser`** — HTML parsing. Uses the `scraper` crate to extract structured
  data from raw HTML.
- **`models`** — Domain types (`Page`, `CodeBlock`, `Menu`, `MenuItem`). Shared
  between client and parser.
- **`error`** — Typed errors using `thiserror`. Callers match on `ErrorKind` to
  handle HTTP, parse, and encoding failures separately.

## Design decisions

- **Blocking HTTP** via `reqwest::blocking`. Simpler API for a scraper library.
  Callers that need async can spawn a thread.
- **No async in public API**. Keeps the interface simple. Internal retry is
  synchronous.
- **Immutable public types**. `Page`, `CodeBlock`, etc. are all owned data with
  public accessors.
