use std::time::Instant;
use dashmap::DashMap;
use crate::storage::in_mem_hashmap::CacheValue;

pub struct Cache {}

impl Cache {
    pub fn get(key: &str, storage: &DashMap<String, CacheValue>) -> Option<String> {
        if let Some(cache_value) = storage.get(key).as_deref() { 
            Some(cache_value.value.to_string())
        }else{
            None
        }
    }
    
    pub fn put(key: &str, val: &str, storage: &DashMap<String, CacheValue>) {
        storage.insert(key.to_string(), CacheValue{ 
            value: val.to_string(), 
            expiry: Instant::now()
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
            Some(cache_value.expiry)
        } else {
            None
        }
    }
}
