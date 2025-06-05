use actix_web::{web, HttpResponse, Responder};
use crate::api::cache;
use crate::api::models::{Entry, Query};
use crate::storage::in_mem_hashmap::AppState;

pub async fn ping() -> impl Responder {
    "pong"
}

pub async fn insert(data: web::Json<Entry>, app_data: web::Data<AppState>) -> impl Responder {
    cache::Cache::put(&data.key, &data.value, &app_data.hash_map);
    HttpResponse::Ok().body("Inserted!")
}

pub async fn retrieve(data: web::Json<Query>, app_data: web::Data<AppState>) -> impl Responder {
    let result = cache::Cache::get(&data.key, &app_data.hash_map);
    HttpResponse::Ok().body(result)
}

pub async fn erase(data: web::Json<Query>, app_data: web::Data<AppState>) -> impl Responder {
    cache::Cache::delete(&data.key, &app_data.hash_map);
    HttpResponse::Ok().body("Erased!")
}