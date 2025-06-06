use std::io::{self, Write};
use std::sync::Arc;
use std::time::Duration;
use crate::app::state::AppState;
use crate::caching::cache::Cache;
use crate::caching::removal::periodic_removal;

pub fn start_cli() {
    // Create a new runtime for CLI mode
    let rt = tokio::runtime::Runtime::new().unwrap();

    let app_state = Arc::new(AppState::get_app_state());
    println!("Welcome to Rustis CLI");
    println!("Type 'help' for available commands");

    // Spawn the periodic removal task in our runtime
    let app_state_clone = Arc::clone(&app_state);
    let handle = rt.handle();
    handle.spawn(async move {
        let interval = Duration::from_secs(20);
        loop {
            tokio::time::sleep(interval).await;
            periodic_removal(app_state_clone.clone());
        }
    });

    loop {
        print!("rustis> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        match parts[0].to_lowercase().as_str() {
            "get" => {
                if parts.len() != 2 {
                    println!("Usage: GET <key>");
                    continue;
                }
                if let Some(value) = Cache::get(parts[1], &app_state.hash_map) {
                    println!("\"{}\"", value);
                } else {
                    println!("(nil)");
                }
            },
            "set" => {
                if parts.len() < 3 {
                    println!("Usage: SET <key> <value> [TTL]");
                    continue;
                }
                let ttl = if parts.len() > 3 {
                    parts[3].parse::<u64>().ok()
                } else {
                    None
                };
                Cache::put(parts[1], parts[2], ttl, &app_state.hash_map);
                println!("OK");
            },
            "del" => {
                if parts.len() != 2 {
                    println!("Usage: DEL <key>");
                    continue;
                }
                Cache::delete(parts[1], &app_state.hash_map);
                println!("OK");
            },
            "keys" => {
                let keys = Cache::get_all(&app_state.hash_map);
                if keys.is_empty() {
                    println!("(empty list or set)");
                } else {
                    for key in keys {
                        println!("1) \"{}\"", key);
                    }
                }
            },
            "ttl" => {
                if parts.len() != 2 {
                    println!("Usage: TTL <key>");
                    continue;
                }
                if let Some(expiry) = Cache::get_expiry(parts[1], &app_state.hash_map) {
                    let remaining = expiry.elapsed().as_secs();
                    println!("{}", remaining);
                } else {
                    println!("-1");
                }
            },
            "help" => {
                println!("Available commands:");
                println!("  GET <key>              - Get value for key");
                println!("  SET <key> <value> [TTL] - Set key to value with optional TTL in seconds");
                println!("  DEL <key>              - Delete key");
                println!("  KEYS                   - List all keys");
                println!("  TTL <key>              - Get remaining TTL for key");
                println!("  HELP                   - Show this help message");
                println!("  EXIT                   - Exit the CLI");
            },
            "exit" => {
                println!("Goodbye!");
                // Give a small grace period for cleanup
                std::thread::sleep(Duration::from_millis(100));
                rt.shutdown_background();
                break;
            },
            _ => {
                println!("Unknown command. Type 'help' for available commands");
            }
        }
    }
}