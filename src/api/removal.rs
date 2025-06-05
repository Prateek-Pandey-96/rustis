use std::sync::Arc;
use std::time::Instant;
use crate::api::cache::Cache;
use crate::storage::in_mem_hashmap::{AppState};

pub fn periodic_removal(app_state: Arc<AppState>) {
    let keys = Cache::get_all(&app_state.hash_map);
    for key in keys {
        let val = Cache::get_expiry(&key, &app_state.hash_map).unwrap();
        if Instant::now() - val > std::time::Duration::from_secs(100) {
            Cache::delete(&key, &app_state.hash_map);
        }
    }
}