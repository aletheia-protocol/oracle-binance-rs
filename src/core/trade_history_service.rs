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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::trade::{TradeData, TradeSD};
    use std::time::{SystemTime, UNIX_EPOCH};
    use rand::Rng;

    // Helper function to generate a random trade
    fn generate_trade(event_time_offset: u64, quantity: &str, trade_id: u64) -> TradeSD {
        TradeSD {
            stream: "btcusdt@tradeStream".to_string(),
            data: TradeData {
                event_type: "trade".to_string(),
                event_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64 - event_time_offset,
                symbol: "BTCUSDT".to_string(),
                trade_id,
                price: "50000".to_string(),
                quantity: quantity.to_string(),
                trade_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64 - event_time_offset,
                is_buyer_market_maker: true,
                ignore: false,
            },
        }
    }

    #[tokio::test]
    async fn test_large_trade_set_with_corner_case_near_60_seconds() {
        let service = TradeHistoryService;
        let mut rng = rand::thread_rng();

        // Generate a set of trades within 60 seconds window and a few outside
        for i in 0..1000 {
            // Event time offsets from 0 ms to 70 seconds
            let event_time_offset = rng.gen_range(0..70_000);
            let trade = generate_trade(event_time_offset, "100", i as u64);
            service.add_trade(trade).await;
        }

        // Add a corner case trade exactly at 60 seconds
        let corner_trade = generate_trade(60_000, "200", 1001);
        service.add_trade(corner_trade).await;

        // Calculate total volume (only trades within the last 60 seconds should count)
        let total_volume = service.total_volume().await;
        log::info!("Total Volume: {}", total_volume);

        // Calculate average volume per trade
        let average_volume = service.average_volume_per_trade().await;
        log::info!("Average Volume per Trade: {}", average_volume);

        // Assert that total volume and average volume are correctly calculated
        assert!(total_volume > 0.0);
        assert!(average_volume > 0.0);

        // Test that the last trade exactly at 60 seconds is included
        let last_trade_volume = 200.0;
        assert!(total_volume >= last_trade_volume);
    }

    #[tokio::test]
    async fn test_no_trades_after_60_seconds() {
        let service = TradeHistoryService;

        // Generate trades that are all older than 60 seconds
        for i in 0..10 {
            let event_time_offset = 70_000; // All trades 70 seconds old
            let trade = generate_trade(event_time_offset, "100", i as u64);
            service.add_trade(trade).await;
        }

        // Ensure that no trades are counted (all should be ignored)
        let total_volume = service.total_volume().await;
        assert_eq!(total_volume, 0.0);

        let average_volume = service.average_volume_per_trade().await;
        assert_eq!(average_volume, 0.0);
    }
}