use crate::core::order_book_service::OrderBookService;
use crate::domain::stream_data::StreamData;
use crate::config::CONFIG;
use binance_spot_connector_rust::market_stream::partial_depth::PartialDepthStream;
use binance_spot_connector_rust::tokio_tungstenite::BinanceWebSocketClient;
use futures_util::StreamExt;
use log;

pub async fn start_websocket() {
    let service = OrderBookService;

    let (mut conn, _) = BinanceWebSocketClient::connect_async_default()
        .await
        .expect("Failed to connect");

    conn.subscribe(vec![
        &PartialDepthStream::from_100ms(CONFIG.default.trading_pair.as_str(), CONFIG.default.book_depth.into()).into()
    ])
        .await;

    while let Some(message) = conn.as_mut().next().await {
        match message {
            Ok(message) => {
                let binary_data = message.into_data();
                if let Ok(data) = std::str::from_utf8(&binary_data) {
                    if !data.contains(":null") {
                        //log::info!("DATA {}",data);
                        if let Ok(result) = serde_json::from_str::<StreamData>(data.trim()) {
                            service.update_order_book(result).await;
                            service.print_top_of_book().await;
                        } else {
                            log::error!("Failed to parse StreamData from JSON: {}", data);
                        }
                    } else {
                        log::info!("Empty row: {}", data);
                    }
                } else {
                    log::error!("Failed to parse message to utf8");
                }
            }
            Err(e) => {
                log::error!("Error receiving message: {}", e);
                break;
            }
        }
    }
    conn.close().await.expect("Failed to disconnect");
}