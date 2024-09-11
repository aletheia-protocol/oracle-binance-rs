use binance_spot_connector_rust::market_stream::book_ticker::BookTickerStream;
use crate::config::CONFIG;
use binance_spot_connector_rust::tokio_tungstenite::BinanceWebSocketClient;
use futures_util::StreamExt;
use log;
use tokio::time::{sleep, Duration};
use crate::core::book_ticker_service::{BookTickerService, BookTickerServiceTrait};
use crate::domain::book_ticker::BookTickerSD;

pub async fn start_websocket() {
    let service = BookTickerService;
    let max_retries = CONFIG.default.ws_config_retry_max;
    let mut retry_count = 0;

    loop {
        match BinanceWebSocketClient::connect_async_default().await {
            Ok((mut conn, _)) => {
                log::info!("WebSocket: BookTicker connection established.");

                conn.subscribe(vec![
                    &BookTickerStream::from_symbol(CONFIG.default.trading_pair.as_str()).into()
                ]).await;

                // Reset retry count on successful connection
                retry_count = 0;

                while let Some(message) = conn.as_mut().next().await {
                    match message {
                        Ok(message) => {
                            let binary_data = message.into_data();
                            if let Ok(data) = std::str::from_utf8(&binary_data) {
                                //log::info!("DATA {}",data);
                               if !data.contains(":null") {
                                   if let Ok(result) = serde_json::from_str::<BookTickerSD>(data.trim()){
                                       service.update_ticker(result).await;
                                       service.print_ticker().await;
                                   }else {
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

                log::info!("Connection closed. Reconnecting...");
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