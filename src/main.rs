use binance_spot_connector_rust::websocket::*;
use serde_json::Value;
use std::collections::BTreeMap;
use tokio::sync::Mutex;
use std::sync::Arc;

// Struktura księgi zamówień (order book)
struct OrderBook {
    bids: BTreeMap<f64, f64>, // Kupno (bids)
    asks: BTreeMap<f64, f64>, // Sprzedaż (asks)
}

impl OrderBook {
    fn new() -> Self {
        OrderBook {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    // Aktualizujemy księgę zamówień na podstawie danych z WebSocketu
    fn update(&mut self, update: &Value) {
        if let Some(bids) = update.get("b") {
            for bid in bids.as_array().unwrap() {
                let price = bid[0].as_str().unwrap().parse::<f64>().unwrap();
                let qty = bid[1].as_str().unwrap().parse::<f64>().unwrap();
                if qty == 0.0 {
                    self.bids.remove(&price);
                } else {
                    self.bids.insert(price, qty);
                }
            }
        }

        if let Some(asks) = update.get("a") {
            for ask in asks.as_array().unwrap() {
                let price = ask[0].as_str().unwrap().parse::<f64>().unwrap();
                let qty = ask[1].as_str().unwrap().parse::<f64>().unwrap();
                if qty == 0.0 {
                    self.asks.remove(&price);
                } else {
                    self.asks.insert(price, qty);
                }
            }
        }
    }

    // Obliczamy imbalance (różnica wolumenu między bid i ask)
    fn calculate_imbalance(&self) -> f64 {
        let total_bid_volume: f64 = self.bids.values().sum();
        let total_ask_volume: f64 = self.asks.values().sum();
        total_bid_volume - total_ask_volume
    }
}

#[tokio::main]
async fn main() {
    // Tworzymy księgę zamówień z synchronizacją między wątkami
    let order_book = Arc::new(Mutex::new(OrderBook::new()));
    let order_book_clone = Arc::clone(&order_book);

    // Inicjalizacja WebSocketu z Binance Spot Connector
    let ws_client = WebSocket::new().expect("Failed to create WebSocket client");

    let callback = move |event: WebSocketEvent| {
        if let WebSocketEvent::Message(msg) = event {
            let data = serde_json::from_str::<Value>(&msg).unwrap();
            let update = data.get("data").unwrap();

            let mut book = tokio::spawn(async move {
                let mut book = order_book_clone.lock().await;
                book.update(update);
                let imbalance = book.calculate_imbalance();
                println!("Imbalance: {}", imbalance);
            });

            tokio::spawn(async move {
                let _ = book.await;
            });
        }
    };

    // Subskrypcja do strumienia głębokości rynku (order book depth) dla pary BTC/USDT
    ws_client
        .subscribe_depth("btcusdt", None, callback)
        .expect("Failed to subscribe to depth stream")
        .connect("wss://stream.binance.com:9443/ws")
        .await
        .expect("WebSocket connection failed");

    // Utrzymujemy połączenie WebSocket
    tokio::signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
}