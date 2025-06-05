mod api;
mod storage;

use std::sync::Arc;
use actix_web::{App, HttpServer, web};
use crate::api::handlers::{erase, insert, ping, retrieve};
use crate::storage::in_mem_hashmap;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let shared_app_state = Arc::new(in_mem_hashmap::AppState::get_app_state());
    
    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/api")
                    .app_data(web::Data::from({
                        Arc::clone(&shared_app_state)
                    }))
                    .route("/ping", web::get().to(ping))
                    .route("/get", web::post().to(retrieve))
                    .route("/set", web::post().to(insert))
                    .route("/del", web::post().to(erase))
            )
    })
        .bind(("127.0.0.1", 6378))?
        .run()
        .await
}