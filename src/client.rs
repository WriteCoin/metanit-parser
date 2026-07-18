use crate::error::{Error, ErrorKind, Result};
use crate::models::Page;
use crate::parser::parse_page;
use log::{info, warn};
use reqwest::blocking::Client as HttpClient;
use reqwest::header::{HeaderMap, USER_AGENT};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

const DEFAULT_CACHE_TTL: Duration = Duration::from_secs(300);

struct CacheEntry {
    body: String,
    #[allow(dead_code)]
    expires_at: Instant,
}

pub struct MetanitClient {
    http: HttpClient,
    cache: Mutex<HashMap<String, CacheEntry>>,
    cache_ttl: Duration,
    max_retries: u32,
}

impl MetanitClient {
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, "metanit-parser/1.0".parse().unwrap());
        let http = HttpClient::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap();
        MetanitClient {
            http,
            cache: Mutex::new(HashMap::new()),
            cache_ttl: DEFAULT_CACHE_TTL,
            max_retries: 3,
        }
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, "metanit-parser/1.0".parse().unwrap());
        self.http = HttpClient::builder()
            .default_headers(headers)
            .timeout(timeout)
            .build()
            .unwrap();
        self
    }

    pub fn with_cache_ttl(mut self, ttl: Duration) -> Self {
        self.cache_ttl = ttl;
        self
    }
    pub fn with_max_retries(mut self, retries: u32) -> Self {
        self.max_retries = retries;
        self
    }
    pub fn clear_cache(&self) {
        self.cache.lock().unwrap().clear();
    }

    pub fn fetch_page(&self, url: &str) -> Result<Page> {
        let body = self.fetch_raw(url)?;
        parse_page(url, &body)
    }

    pub fn fetch_raw(&self, url: &str) -> Result<String> {
        if let Some(cached) = self.check_cache(url) {
            info!("Cache hit: {}", url);
            return Ok(cached);
        }
        info!("Fetching: {}", url);
        let mut last_error = None;
        for attempt in 0..=self.max_retries {
            match self.try_fetch(url) {
                Ok(body) => {
                    self.set_cache(url, body.clone());
                    return Ok(body);
                }
                Err(e) => {
                    warn!("Attempt {} failed: {}", attempt + 1, e);
                    last_error = Some(e);
                    if attempt < self.max_retries {
                        std::thread::sleep(Duration::from_millis(500 * u64::from(attempt + 1)));
                    }
                }
            }
        }
        Err(last_error
            .unwrap_or_else(|| Error::new(ErrorKind::Http, format!("Failed to fetch {}", url))))
    }

    fn try_fetch(&self, url: &str) -> Result<String> {
        let response = self.http.get(url).send()?;
        if !response.status().is_success() {
            return Err(Error::new(
                ErrorKind::Http,
                format!("HTTP {}", response.status()),
            ));
        }
        Ok(response.text()?)
    }

    fn check_cache(&self, url: &str) -> Option<String> {
        self.cache.lock().ok()?.get(url).map(|e| e.body.clone())
    }

    fn set_cache(&self, url: &str, body: String) {
        if let Ok(mut cache) = self.cache.lock() {
            cache.insert(
                url.to_string(),
                CacheEntry {
                    body,
                    expires_at: Instant::now() + self.cache_ttl,
                },
            );
        }
    }
}

impl Default for MetanitClient {
    fn default() -> Self {
        Self::new()
    }
}
