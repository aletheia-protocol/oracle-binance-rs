use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BookTickerSD {
    pub stream: String,      // Stream name (e.g., btcfdusd@bookTicker)
    pub data: BookTickerData, // BookTicker data
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BookTickerData {
    #[serde(rename = "u")]
    pub update_id: u64,      // Update ID
    #[serde(rename = "s")]
    pub symbol: String,   // Symbol (e.g., BTCFDUSD)
    #[serde(rename = "b")]
    pub best_bid_price: String,   // Best bid price
    #[serde(rename = "B")]
    pub best_bid_qty: String,   // Best bid quantity
    #[serde(rename = "a")]
    pub best_ask_price: String,   // Best ask price
    #[serde(rename = "A")]
    pub best_ask_qty: String,   // Best ask quantity
}

impl BookTickerData {
    // Method to print the data in a formatted way
    pub fn print(&self) {
        log::info!("Symbol: {}", self.symbol);
        log::info!("Update ID: {}", self.update_id);
        log::info!("Best Bid: {} @ {}", self.best_bid_qty, self.best_bid_price);
        log::info!("Best Ask: {} @ {}", self.best_ask_qty, self.best_ask_price);
    }

    pub fn mid_price(&self) -> f64 {
        let best_bid = self.best_bid_price.parse::<f64>().unwrap_or(0.0);
        let best_ask = self.best_ask_price.parse::<f64>().unwrap_or(0.0);
        (best_bid + best_ask) / 2.0
    }

    // Method to calculate the mid-weighted price (weighted by bid and ask quantities)
    pub fn mid_weighted_price(&self) -> f64 {
        let best_bid = self.best_bid_price.parse::<f64>().unwrap_or(0.0);
        let best_ask = self.best_ask_price.parse::<f64>().unwrap_or(0.0);
        let bid_qty = self.best_bid_qty.parse::<f64>().unwrap_or(0.0);
        let ask_qty = self.best_ask_qty.parse::<f64>().unwrap_or(0.0);

        if bid_qty + ask_qty == 0.0 {
            return 0.0; // Avoid division by zero
        }

        // Weighted average formula
        ((best_bid * bid_qty) + (best_ask * ask_qty)) / (bid_qty + ask_qty)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mid_price() {
        let data = BookTickerData {
            update_id: 123,
            symbol: "BTCUSD".to_string(),
            best_bid_price: "50000.0".to_string(),
            best_bid_qty: "2.0".to_string(),
            best_ask_price: "51000.0".to_string(),
            best_ask_qty: "3.0".to_string(),
        };

        let mid_price = data.mid_price();
        assert_eq!(mid_price, 50500.0); // (50000 + 51000) / 2
    }

    #[test]
    fn test_mid_weighted_price() {
        let data = BookTickerData {
            update_id: 123,
            symbol: "BTCUSD".to_string(),
            best_bid_price: "50000.0".to_string(),
            best_bid_qty: "2.0".to_string(),
            best_ask_price: "51000.0".to_string(),
            best_ask_qty: "3.0".to_string(),
        };

        let mid_weighted_price = data.mid_weighted_price();
        // Weighted average: (50000 * 2 + 51000 * 3) / (2 + 3) = 50600
        assert_eq!(mid_weighted_price, 50600.0);
    }

    #[test]
    fn test_mid_weighted_price_with_zero_quantities() {
        let data = BookTickerData {
            update_id: 123,
            symbol: "BTCUSD".to_string(),
            best_bid_price: "50000.0".to_string(),
            best_bid_qty: "0.0".to_string(),
            best_ask_price: "51000.0".to_string(),
            best_ask_qty: "0.0".to_string(),
        };

        let mid_weighted_price = data.mid_weighted_price();
        // Both bid and ask quantities are zero, should return 0.0
        assert_eq!(mid_weighted_price, 0.0);
    }

    #[test]
    fn test_mid_price_with_zero_prices() {
        let data = BookTickerData {
            update_id: 123,
            symbol: "BTCUSD".to_string(),
            best_bid_price: "0.0".to_string(),
            best_bid_qty: "2.0".to_string(),
            best_ask_price: "0.0".to_string(),
            best_ask_qty: "3.0".to_string(),
        };

        let mid_price = data.mid_price();
        // Both bid and ask prices are zero, mid price should be 0.0
        assert_eq!(mid_price, 0.0);
    }
}