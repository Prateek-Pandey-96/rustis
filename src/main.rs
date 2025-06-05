mod api;
mod storage;

use std::sync::Arc;
use std::time::{Duration, Instant};
use actix_web::{App, HttpServer, web};
use crate::api::handlers::{erase, insert, ping, retrieve, retrieve_all};
use crate::api::removal::periodic_removal;
use crate::storage::in_mem_hashmap;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let shared_app_state = Arc::new(in_mem_hashmap::AppState::get_app_state());

    let shared_app_state_clone = Arc::clone(&shared_app_state);
    tokio::spawn( async move {
        let interval = Duration::from_secs(20);
        loop{
            tokio::time::sleep(interval).await;
            periodic_removal(shared_app_state_clone.clone())
        }
    });
    
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
                    .route("/keys", web::get().to(retrieve_all))
            )
    })
        .bind(("127.0.0.1", 6378))?
        .run()
        .await
}