use actix_web::{web, HttpResponse, Responder};
use crate::caching::cache::Cache;
use crate::caching::models::{Entry, Query};
use super::state::AppState;

pub async fn ping() -> impl Responder {
    "pong"
}

pub async fn insert(data: web::Json<Entry>, app_data: web::Data<AppState>) -> impl Responder {
    Cache::put(&data.key, &data.value, data.ttl, &app_data.hash_map);
    HttpResponse::Ok().body("Inserted!")
}

pub async fn retrieve(data: web::Json<Query>, app_data: web::Data<AppState>) -> impl Responder {
    let result = Cache::get(&data.key, &app_data.hash_map);
    if result.is_none() {
        return HttpResponse::NotFound().body("nil");
    }
    HttpResponse::Ok().body(result.unwrap())
}

pub async fn erase(data: web::Json<Query>, app_data: web::Data<AppState>) -> impl Responder {
    Cache::delete(&data.key, &app_data.hash_map);
    HttpResponse::Ok().body("Erased!")
}

pub async fn retrieve_all(app_data: web::Data<AppState>) -> impl Responder{
    let keys = Cache::get_all(&app_data.hash_map);
    HttpResponse::Ok().json(keys)
}