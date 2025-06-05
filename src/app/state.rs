use std::time::Instant;
use dashmap::DashMap;

pub struct CacheValue {
    pub value: String,
    pub expiry: Option<Instant>,
}

pub struct AppState{
    pub hash_map: DashMap<String, CacheValue>
}

impl AppState {
    pub fn get_app_state() -> AppState {
       AppState{
           hash_map: DashMap::new()
       }
    }
}
