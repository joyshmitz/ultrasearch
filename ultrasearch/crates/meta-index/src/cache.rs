use std::num::NonZeroUsize;
use std::sync::Arc;

use ahash::RandomState;
use core_types::DocKey;
use lasso::Rodeo;
use lru::LruCache;
use slotmap::{SlotMap, new_key_type};
use std::collections::HashMap;
use core_types::{FileMeta, FileFlags, Timestamp};
use lasso::Spur;

new_key_type! { pub struct CacheKey; }

/// Minimal in-memory cache for metadata acceleration and path reconstruction.
pub struct MetadataCache {
    /// Primary storage for cached file items.
    slots: SlotMap<CacheKey, CachedItem>,
    /// Lookup map from stable DocKey to internal SlotKey.
    lookup: HashMap<DocKey, CacheKey>,
    /// LRU cache for fully resolved paths.
    path_cache: LruCache<DocKey, Arc<str>, RandomState>,
    /// String interner for filenames to save memory.
    interner: Rodeo,
}

/// Compact representation of a file in the cache.
#[derive(Debug, Clone)]
pub struct CachedItem {
    pub key: DocKey,
    pub parent: Option<DocKey>,
    pub name: Spur,
    pub size: u64,
    pub modified: Timestamp,
    pub flags: FileFlags,
}

impl MetadataCache {
    pub fn new(path_capacity: usize) -> Self {
        let cap = NonZeroUsize::new(path_capacity).unwrap_or(NonZeroUsize::new(1000).unwrap());
        let s = RandomState::new();
        Self {
            slots: SlotMap::with_key(),
            lookup: HashMap::new(),
            path_cache: LruCache::with_hasher(cap, s),
            interner: Rodeo::new(),
        }
    }

    pub fn put(&mut self, meta: &FileMeta) {
        self.path_cache.pop(&meta.key);
        let name_spur = self.interner.get_or_intern(&meta.name);

        if let Some(&slot_key) = self.lookup.get(&meta.key) {
            if let Some(item) = self.slots.get_mut(slot_key) {
                item.parent = meta.parent;
                item.name = name_spur;
                item.size = meta.size;
                item.modified = meta.modified;
                item.flags = meta.flags;
            }
        } else {
            let item = CachedItem {
                key: meta.key,
                parent: meta.parent,
                name: name_spur,
                size: meta.size,
                modified: meta.modified,
                flags: meta.flags,
            };
            let slot_key = self.slots.insert(item);
            self.lookup.insert(meta.key, slot_key);
        }
    }

    pub fn remove(&mut self, key: DocKey) {
        self.path_cache.pop(&key);
        if let Some(slot_key) = self.lookup.remove(&key) {
            self.slots.remove(slot_key);
        }
    }

    pub fn get(&self, key: DocKey) -> Option<&CachedItem> {
        self.lookup.get(&key).and_then(|&slot| self.slots.get(slot))
    }

    pub fn resolve_path(&mut self, key: DocKey) -> Option<Arc<str>> {
        if let Some(path) = self.path_cache.get(&key) {
            return Some(path.clone());
        }

        let mut current_key = key;
        let mut segments = Vec::new();

        loop {
            if let Some(item) = self.get(current_key) {
                let name_str = self.interner.resolve(&item.name);
                segments.push(name_str);

                if let Some(parent) = item.parent {
                    if parent == current_key { break; }
                    current_key = parent;
                } else {
                    break;
                }
            } else {
                return None;
            }
        }

        segments.reverse();
        let full_path = segments.join(std::path::MAIN_SEPARATOR_STR);
        let path_arc: Arc<str> = full_path.into();
        
        self.path_cache.put(key, path_arc.clone());
        Some(path_arc)
    }
    
    pub fn clear(&mut self) {
        self.slots.clear();
        self.lookup.clear();
        self.path_cache.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core_types::FileFlags;

    fn make_meta(key: DocKey, parent: Option<DocKey>, name: &str) -> FileMeta {
        FileMeta::new(
            key, 0, parent, name.to_string(), None, 100, 0, 0, FileFlags::empty(),
        )
    }

    #[test]
    fn test_cache_put_get_remove() {
        let mut cache = MetadataCache::new(10);
        let key = DocKey::from_parts(1, 100);
        let meta = make_meta(key, None, "test.txt");

        cache.put(&meta);
        assert!(cache.get(key).is_some());
        let cached = cache.get(key).unwrap();
        assert_eq!(cache.interner.resolve(&cached.name), "test.txt");

        cache.remove(key);
        assert!(cache.get(key).is_none());
    }

    #[test]
    fn test_path_reconstruction() {
        let mut cache = MetadataCache::new(10);
        let root_key = DocKey::from_parts(1, 1);
        let dir_key = DocKey::from_parts(1, 2);
        let file_key = DocKey::from_parts(1, 3);

        cache.put(&make_meta(root_key, None, "C:"));
        cache.put(&make_meta(dir_key, Some(root_key), "Users"));
        cache.put(&make_meta(file_key, Some(dir_key), "test.txt"));

        let path = cache.resolve_path(file_key).expect("should resolve");
        #[cfg(windows)]
        assert_eq!(&*path, "C:\\Users\\test.txt");
        #[cfg(not(windows))]
        assert_eq!(&*path, "C:/Users/test.txt");
    }
}