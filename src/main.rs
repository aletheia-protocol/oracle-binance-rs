mod domain;
mod core;
mod adapters;
mod ports;
mod config;

use crate::ports::ws_client_order_book::start_websocket;
use crate::adapters::rest_api::create_rest_api;
use crate::config::CONFIG;

#[tokio::main]
async fn main() {
    // Initialize the logger at the very start of the program
    env_logger::init();

    // Log the start of the application
    log::info!("Starting application...");

    // Start both WebSocket and REST API in the same Tokio runtime using join!
    let websocket_handle = tokio::spawn(async {
        log::info!("Starting WebSocket client...");
        start_websocket().await;
    });

    let rest_api_handle = tokio::spawn(async {
        log::info!("Starting REST API server on port {}...", CONFIG.default.server_port);
        let api = create_rest_api();
        warp::serve(api).run(([0, 0, 0, 0], CONFIG.default.server_port)).await;
    });

    // Wait for both tasks to finish (if they ever finish)
    tokio::try_join!(websocket_handle, rest_api_handle).unwrap();
}