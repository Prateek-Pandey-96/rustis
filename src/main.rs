mod caching;
mod app;
mod cli;

use std::sync::Arc;
use std::time::Duration;
use actix_web::{App, HttpServer, web};
use app::handlers::{erase, insert, ping, retrieve, retrieve_all};
use crate::caching::removal::periodic_removal;
use crate::app::state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    // Check if we should run in CLI mode
    if args.len() > 1 && args[1] == "--cli" {
        cli::start_cli();
        return Ok(());
    }
    
    // Create app state to be used by each thread of actix and also by keys eliminator
    let shared_app_state = Arc::new(state::AppState::get_app_state());

    // Periodically clean expired keys
    let shared_app_state_clone = Arc::clone(&shared_app_state);
    tokio::spawn(async move {
        let interval = Duration::from_secs(20);
        loop {
            tokio::time::sleep(interval).await;
            periodic_removal(Arc::clone(&shared_app_state_clone));
        }
    });

    // Start the server
    println!("Starting Rustis server on 127.0.0.1:6378");
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