mod domain;
mod core;
mod adapters;
mod ports;

use std::thread;

use crate::ports::ws_client_order_book::start_websocket;
use crate::adapters::rest_api::create_rest_api;


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
            warp::serve(api).run(([127, 0, 0, 1], 3030)).await;
        });
    });

    websocket_thread.join().unwrap();
}