use std::time::Duration;
use reqwest::blocking::Client as HttpClient;
use reqwest::header::{HeaderMap, USER_AGENT};
use crate::error::{Error, ErrorKind, Result};
use crate::models::Page;
use crate::parser::parse_page;

pub struct MetanitClient {
    http: HttpClient,
}

impl MetanitClient {
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, "metanit-parser/1.0".parse().unwrap());
        let http = HttpClient::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(30))
            .build().unwrap();
        MetanitClient { http }
    }

    pub fn fetch_page(&self, url: &str) -> Result<Page> {
        let body = self.fetch_raw(url)?;
        parse_page(url, &body)
    }

    pub fn fetch_raw(&self, url: &str) -> Result<String> {
        let response = self.http.get(url).send()?;
        if !response.status().is_success() {
            return Err(Error::new(ErrorKind::Http, format!("HTTP {}", response.status())));
        }
        Ok(response.text()?)
    }
}

impl Default for MetanitClient { fn default() -> Self { Self::new() } }
