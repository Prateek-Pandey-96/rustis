use dashmap::DashMap;

pub struct Cache {}

impl Cache {
    pub fn get(key: &str, storage: &DashMap<String, String>) -> String {
        let mut result = "nil".to_string();
        if let Some(value) = storage.get(key).as_deref() {
            result = value.to_string();
        }
        result
    }
    
    pub fn put(key: &str, val: &str, storage: &DashMap<String, String>) {
        storage.insert(key.to_string(), val.to_string());
    }
    
    pub fn delete(key: &str, storage: &DashMap<String, String>) {
        storage.remove(key);
    }
}
