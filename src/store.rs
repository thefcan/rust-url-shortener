//! Persistence abstraction for short links, with an in-memory implementation.
use std::collections::HashMap;
use std::sync::RwLock;

/// A stored short link: its target URL and how many times it has been followed.
#[derive(Clone)]
pub struct Link {
    pub url: String,
    pub hits: u64,
}

/// Store abstracts how short links are kept, so the HTTP layer is agnostic to
/// the backend (in-memory now; a database could implement the same trait).
pub trait Store: Send + Sync {
    /// Save `url` under `code`.
    fn save(&self, code: String, url: String);
    /// Resolve a code to its URL, counting a hit. Returns None if unknown.
    fn resolve(&self, code: &str) -> Option<String>;
}

/// In-memory Store backed by a read/write-locked map.
#[derive(Default)]
pub struct MemoryStore {
    links: RwLock<HashMap<String, Link>>,
}

impl MemoryStore {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Store for MemoryStore {
    fn save(&self, code: String, url: String) {
        self.links
            .write()
            .unwrap()
            .insert(code, Link { url, hits: 0 });
    }

    fn resolve(&self, code: &str) -> Option<String> {
        let mut links = self.links.write().unwrap();
        let link = links.get_mut(code)?;
        link.hits += 1;
        Some(link.url.clone())
    }
}
