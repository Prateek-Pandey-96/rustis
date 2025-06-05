use std::sync::Arc;
use std::time::Instant;
use super::cache::Cache;
use crate::app::state::{AppState};

pub fn periodic_removal(app_state: Arc<AppState>) {
    let keys = Cache::get_all(&app_state.hash_map);
    for key in keys {
        if let Some(expiry) = Cache::get_expiry(&key, &app_state.hash_map) {
            if expiry < Instant::now() {
                Cache::delete(&key, &app_state.hash_map);
            }
        }
    }
}