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
    /// Fetch a link's metadata without counting a hit.
    fn get(&self, code: &str) -> Option<Link>;
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

    fn get(&self, code: &str) -> Option<Link> {
        self.links.read().unwrap().get(code).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_then_get_returns_link_with_zero_hits() {
        let store = MemoryStore::new();
        store.save("abc".into(), "https://example.com/".into());

        let link = store.get("abc").expect("saved link should exist");
        assert_eq!(link.url, "https://example.com/");
        assert_eq!(link.hits, 0);
    }

    #[test]
    fn get_is_none_for_unknown_code() {
        let store = MemoryStore::new();
        assert!(store.get("missing").is_none());
    }

    #[test]
    fn resolve_returns_url_and_counts_each_hit() {
        let store = MemoryStore::new();
        store.save("abc".into(), "https://example.com/".into());

        assert_eq!(
            store.resolve("abc").as_deref(),
            Some("https://example.com/")
        );
        assert_eq!(
            store.resolve("abc").as_deref(),
            Some("https://example.com/")
        );

        assert_eq!(store.get("abc").unwrap().hits, 2);
    }

    #[test]
    fn resolve_is_none_for_unknown_code() {
        let store = MemoryStore::new();
        assert!(store.resolve("nope").is_none());
    }

    #[test]
    fn get_does_not_count_a_hit() {
        let store = MemoryStore::new();
        store.save("abc".into(), "https://example.com/".into());

        store.get("abc");
        store.get("abc");

        assert_eq!(store.get("abc").unwrap().hits, 0);
    }

    #[test]
    fn save_overwrites_url_and_resets_hits() {
        let store = MemoryStore::new();
        store.save("abc".into(), "https://old.example/".into());
        store.resolve("abc");

        store.save("abc".into(), "https://new.example/".into());

        let link = store.get("abc").unwrap();
        assert_eq!(link.url, "https://new.example/");
        assert_eq!(link.hits, 0);
    }

    #[test]
    fn distinct_codes_track_hits_independently() {
        let store = MemoryStore::new();
        store.save("a".into(), "https://a.example/".into());
        store.save("b".into(), "https://b.example/".into());

        store.resolve("a");

        assert_eq!(store.get("a").unwrap().hits, 1);
        assert_eq!(store.get("b").unwrap().hits, 0);
    }
}
