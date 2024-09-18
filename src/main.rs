mod domain;
mod adapters;
mod ports;
mod config;

use std::sync::Arc;
use tonic::transport::Server;
use warp::Filter;
use crate::adapters::proto::book_ticker_proto_service;
use crate::ports::{ws_client_order_book, ws_client_trade};
use crate::ports::ws_client_book_ticker;
use crate::adapters::rest::order_book_api::create_order_book_api;
use crate::adapters::rest::book_ticker_api::create_book_ticker_rest_api;
use crate::adapters::rest::trade_history_rest::create_trade_history_rest_api;
use crate::config::CONFIG;
use crate::domain::services::book_ticker_service::BookTickerService;

#[tokio::main]
async fn main() {
    // Initialize the logger at the very start of the program
    env_logger::init();

    // Log the start of the application
    log::info!("Starting application...");

    let book_ticker_service = Arc::new(BookTickerService);

    // Start both WebSocket and REST API in the same Tokio runtime using join!
    let websocket_order_book_handle = tokio::spawn(async {
        log::info!("Starting OrderBook WebSocket client...");
        ws_client_order_book::start_websocket().await;
    });

    // Start both WebSocket and REST API in the same Tokio runtime using join!
    let websocket_book_ticker_handle = tokio::spawn(async {
        log::info!("Starting BookTicker WebSocket client...");
        ws_client_book_ticker::start_websocket().await;
    });

    let websocket_trade_handle = tokio::spawn( async{
        log::info!("Starting Trade Stream WebSocket client...");
        ws_client_trade::start_websocket().await;
    });


    let rest_api_handle = tokio::spawn(async {
        log::info!("Starting REST API server on port {} ...", CONFIG.default.server_port);
        let api = create_order_book_api()
            .or(create_book_ticker_rest_api())
            .or(create_trade_history_rest_api());
        warp::serve(api).run(([0, 0, 0, 0], CONFIG.default.server_port)).await;
    });

    let grpc_service_handle = tokio::spawn(async move {
        let addr = "[::1]:50051".parse().unwrap(); // Port dla serwisu gRPC
        let grpc_service = book_ticker_proto_service::create_book_ticker_service(book_ticker_service.clone());

        log::info!("Starting gRPC BookTicker service on {}", addr);
        Server::builder()
            .add_service(grpc_service)
            .serve(addr)
            .await
            .unwrap();
    });

    // Wait for both tasks to finish (if they ever finish)
    tokio::try_join!(
        websocket_order_book_handle,
        websocket_book_ticker_handle,
        websocket_trade_handle,
        rest_api_handle,
        grpc_service_handle).unwrap();

    //gRPC

}