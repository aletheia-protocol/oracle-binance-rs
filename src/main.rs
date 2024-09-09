mod domain;

use std::collections::BTreeMap;
use std::sync::Arc;
use binance_spot_connector_rust::{
    market_stream::partial_depth::PartialDepthStream,
    tokio_tungstenite::BinanceWebSocketClient,
};
use env_logger::Builder;
use futures_util::StreamExt;
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use domain::stream_data::StreamData;
use crate::domain::order_book::OrderBook;

#[tokio::main]
async fn main() {

    let order_book = Arc::new(Mutex::new(OrderBook::new()));

    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();
    // Establish connection
    let (mut conn, _) = BinanceWebSocketClient::connect_async_default()
        .await
        .expect("Failed to connect");
    // Subscribe to streams
    conn.subscribe(vec![
        &PartialDepthStream::from_100ms("BTCFDUSD",5).into()
    ])
        .await;
    // Read messages
    while let Some(message) = conn.as_mut().next().await {
        match message {
            Ok(message) => {
                let binary_data = message.into_data();
                let data = std::str::from_utf8(&binary_data).expect("Failed to parse message");
                //let wrapped_data = format!("r#{}#",data);
                if !data.contains(":null")
                {
                    let result: StreamData = serde_json::from_str(format!(r#"{}"#,data).trim()).unwrap();
                    let mut book = order_book.lock().await;
                    book.update(result);
                    book.print_top_of_book();
                }
                else {
                    log::info!("Empty row: {}",data);
                }
            }
            Err(_) => break,
        }
    }
    // Disconnect
    conn.close().await.expect("Failed to disconnect");
}