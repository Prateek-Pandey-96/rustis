use actix_web::App;
use dashmap::DashMap;

pub struct AppState{
    pub hash_map: DashMap<String, String>
}

impl AppState {
    pub fn get_app_state() -> AppState {
       AppState{
           hash_map: DashMap::new()
       }
    }
}
