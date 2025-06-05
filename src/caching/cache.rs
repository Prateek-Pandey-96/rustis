use std::time::{Duration, Instant};
use dashmap::DashMap;
use crate::app::state::CacheValue;

pub struct Cache {}

impl Cache {
    pub fn get(key: &str, storage: &DashMap<String, CacheValue>) -> Option<String> {
        if let Some(cache_value) = storage.get(key).as_deref() {
            Some(cache_value.value.to_string())
        }else{
            None
        }
    }
    
    pub fn put(key: &str, val: &str, ttl: Option<u64>, storage: &DashMap<String, CacheValue>) {
        let expiry = ttl.map(|t| Instant::now() + Duration::from_secs(t));
        storage.insert(key.to_string(), CacheValue{
            value: val.to_string(),
            expiry
        });
    }
    
    pub fn delete(key: &str, storage: &DashMap<String, CacheValue>) {
        storage.remove(key);
    }

    pub fn get_all(storage: &DashMap<String, CacheValue>) -> Vec<String>{
        let mut keys: Vec<String> = Vec::new();
        for entry in storage.iter() {
            let key = entry.key().to_string();
            keys.push(key);
        }
        keys
    }

    pub fn get_expiry(key: &str, storage: &DashMap<String, CacheValue>) -> Option<Instant> {
        if let Some(cache_value) = storage.get(key).as_deref() {
            if let Some(expiry) = cache_value.expiry{
                Some(expiry)
            } else {
                None
            }
        } else {
            None
        }
    }
}
