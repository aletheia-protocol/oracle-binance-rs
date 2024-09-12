use std::collections::VecDeque;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use crate::domain::trade::{TradeData, TradeSD};

// Static singleton for global trade history storage
pub static TRADE_HISTORY: Lazy<Arc<Mutex<VecDeque<TradeData>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(VecDeque::new()))
});

#[derive(Debug, Clone, Default)]
pub struct TradeHistoryService;

impl TradeHistoryService {
    // Add a trade to the rolling window
    pub async fn add_trade(&self, trade_sd: TradeSD) {
        let mut trades = TRADE_HISTORY.lock().await;

        // Get the current time in milliseconds since the UNIX epoch
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;

        // Keep trades within a larger window (e.g., 70 seconds)
        while let Some(oldest_trade) = trades.front() {
            if current_time - oldest_trade.trade_time > 70_000 {
                trades.pop_front();
            } else {
                break;
            }
        }

        // Add the new trade to the rolling window
        trades.push_back(trade_sd.data);
    }

    // Calculate the average volume per trade within the last 60 seconds
    pub async fn average_volume_per_trade(&self) -> f64 {
        let trades = TRADE_HISTORY.lock().await;

        let relevant_trades: Vec<&TradeData> = trades.iter()
            .filter(|trade| self.is_within_last_60_seconds(trade))
            .collect();

        // Return 0 if there are no relevant trades
        if relevant_trades.is_empty() {
            return 0.0;
        }

        // Sum the volumes of all relevant trades and calculate the average
        let total_volume: f64 = relevant_trades.iter()
            .map(|trade| trade.quantity.parse::<f64>().unwrap_or(0.0))
            .sum();
        let trade_count = relevant_trades.len() as f64;

        total_volume / trade_count
    }

    // Calculate the total volume of all trades within the last 60 seconds
    pub async fn total_volume(&self) -> f64 {
        let trades = TRADE_HISTORY.lock().await;

        let relevant_trades: Vec<&TradeData> = trades.iter()
            .filter(|trade| self.is_within_last_60_seconds(trade))
            .collect();

        // Sum the volumes of all relevant trades
        relevant_trades.iter()
            .map(|trade| trade.quantity.parse::<f64>().unwrap_or(0.0))
            .sum()
    }

    // Helper function to check if a trade is within the last 60 seconds
    fn is_within_last_60_seconds(&self, trade: &TradeData) -> bool {
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
        current_time - trade.trade_time <= 60_000
    }
}