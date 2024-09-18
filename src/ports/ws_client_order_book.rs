use crate::domain::services::order_book_service::{OrderBookService, OrderBookServiceTrait};
use crate::domain::entities::order_book::OrderBookSD;
use crate::config::CONFIG;
use binance_spot_connector_rust::market_stream::partial_depth::PartialDepthStream;
use binance_spot_connector_rust::tokio_tungstenite::BinanceWebSocketClient;
use futures_util::StreamExt;
use log;
use tokio::time::{sleep, Duration};

pub async fn start_websocket() {
    let service = OrderBookService;
    let max_retries = CONFIG.default.ws_config_retry_max; // Maximum retries for reconnect
    let mut retry_count = 0;

    loop {
        match BinanceWebSocketClient::connect_async_default().await {
            Ok((mut conn, _)) => {
                log::info!("WebSocket: OrderBook connection established.");

                conn.subscribe(vec![
                    &PartialDepthStream::from_100ms(
                        CONFIG.default.trading_pair.as_str(),
                        CONFIG.default.book_depth
                    ).into()
                ]).await;

                // Reset retry count on successful connection
                retry_count = 0;

                while let Some(message) = conn.as_mut().next().await {
                    match message {
                        Ok(message) => {
                            let binary_data = message.into_data();
                            if let Ok(data) = std::str::from_utf8(&binary_data) {
                                if !data.contains(":null") {
                                    if let Ok(result) = serde_json::from_str::<OrderBookSD>(data.trim()) {
                                        service.update_order_book(result).await;
                                        //service.print_top_of_book().await;
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

                log::info!("WebSocket: OrderBook Connection closed. Reconnecting...");
            }
            Err(e) => {
                retry_count += 1;
                log::error!("Failed to connect to WebSocket: {}. Retry {}/{}", e, retry_count, max_retries);

                if retry_count >= max_retries {
                    log::error!("Max retries reached. Exiting...");
                    break;
                }

                // Wait before attempting to reconnect
                sleep(Duration::from_secs(5)).await;
            }
        }
    }
}