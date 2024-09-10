mod domain;
mod core;
mod adapters;
mod ports;
mod config;

use std::thread;

use crate::ports::ws_client_order_book::start_websocket;
use crate::adapters::rest_api::create_rest_api;
use crate::config::CONFIG;


#[tokio::main]
async fn main() {

    let websocket_thread = thread::spawn(|| {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            start_websocket().await;
        });
    });

    let rest_api_thread = thread::spawn(|| {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let api = create_rest_api();
            warp::serve(api).run(([127, 0, 0, 1], CONFIG.default.server_port.into())).await;
        });
    });

    websocket_thread.join().unwrap();
}