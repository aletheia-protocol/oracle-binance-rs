use std::collections::BTreeMap;
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use crate::domain::stream_data::StreamData;

// Struct representing the order book with bids and asks
pub struct OrderBook {
    pub bids: BTreeMap<OrderedFloat<f64>, f64>,
    pub asks: BTreeMap<OrderedFloat<f64>, f64>,
}

// Struct representing a single entry in the order book (price and quantity)
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderBookEntry {
    pub price: f64,
    pub qty: f64,
}

// Struct representing the top of the order book (best bid and best ask)
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderBookTop {
    pub best_bid: OrderBookEntry,
    pub best_ask: OrderBookEntry,
}

impl OrderBook {
    // Create a new, empty OrderBook
    pub fn new() -> Self {
        OrderBook {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    // Update the order book with new data from a StreamData instance
    pub fn update(&mut self, update: StreamData) {
        let mut new_bids = BTreeMap::new();
        for row in update.data.bids {
            let price = row[0].parse::<f64>().unwrap();
            let qty = row[1].parse::<f64>().unwrap();
            if qty > 0.0 {
                new_bids.insert(OrderedFloat(price), qty);
            }
        }
        self.bids = new_bids;

        let mut new_asks = BTreeMap::new();
        for row in update.data.asks {
            let price = row[0].parse::<f64>().unwrap();
            let qty = row[1].parse::<f64>().unwrap();
            if qty > 0.0 {
                new_asks.insert(OrderedFloat(price), qty);
            }
        }
        self.asks = new_asks;
    }

    // Print the top bid and ask prices and quantities in the order book (for debugging purposes)
    pub fn print_top_of_book(&self) {
        if let Some((&best_bid_price, &best_bid_qty)) = self.bids.iter().next_back() {
            log::info!("Best Bid: {} @ {}", best_bid_qty, best_bid_price);
        }
        if let Some((&best_ask_price, &best_ask_qty)) = self.asks.iter().next() {
            log::info!("Best Ask: {} @ {}", best_ask_qty, best_ask_price);
        }
    }

    // Get the top of the order book (best bid and best ask)
    pub fn get_top(&self) -> Option<OrderBookTop> {
        let best_bid = self.bids.iter().next_back();
        let best_ask = self.asks.iter().next();

        match (best_bid, best_ask) {
            (Some((&best_bid_price, &best_bid_qty)), Some((&best_ask_price, &best_ask_qty))) => {
                Some(OrderBookTop {
                    best_bid: OrderBookEntry {
                        price: best_bid_price.into_inner(),
                        qty: best_bid_qty,
                    },
                    best_ask: OrderBookEntry {
                        price: best_ask_price.into_inner(),
                        qty: best_ask_qty,
                    },
                })
            }
            _ => None, // Empty order book
        }
    }

    // Get the full order book (all bids and asks) as two vectors
    pub fn get_full_book(&self) -> (Option<Vec<OrderBookEntry>>, Option<Vec<OrderBookEntry>>) {
        let bids: Option<Vec<OrderBookEntry>> = if self.bids.is_empty() {
            None
        } else {
            Some(self.bids.iter().map(|(&price, &qty)| OrderBookEntry {
                price: price.into_inner(),
                qty,
            }).collect())
        };

        let asks: Option<Vec<OrderBookEntry>> = if self.asks.is_empty() {
            None
        } else {
            Some(self.asks.iter().map(|(&price, &qty)| OrderBookEntry {
                price: price.into_inner(),
                qty,
            }).collect())
        };

        (bids, asks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::stream_data::{StreamData, DepthData};

    // Test updating the order book and getting the top (best bid and ask)
    #[test]
    fn test_order_book_update_and_get_top() {
        let mut order_book = OrderBook::new();

        // Sample stream data for testing
        let stream_data = StreamData {
            stream: "orderBookUpdate".to_string(),
            data: DepthData {
                last_update_id: 1,
                bids: vec![["10000.0".to_string(), "1.0".to_string()]], // 1 bid at 10000.0
                asks: vec![["10100.0".to_string(), "2.0".to_string()]], // 1 ask at 10100.0
            },
        };

        // Update the order book
        order_book.update(stream_data);

        // Check if the top bid and ask are correctly set
        let top = order_book.get_top().unwrap();
        assert_eq!(top.best_bid.price, 10000.0);
        assert_eq!(top.best_bid.qty, 1.0);
        assert_eq!(top.best_ask.price, 10100.0);
        assert_eq!(top.best_ask.qty, 2.0);
    }

    // Test getting the full order book with multiple bids and asks
    #[test]
    fn test_order_book_get_full_book() {
        let mut order_book = OrderBook::new();

        // Sample stream data for testing
        let stream_data = StreamData {
            stream: "orderBookUpdate".to_string(),
            data: DepthData {
                last_update_id: 1,
                bids: vec![
                    ["10000.0".to_string(), "1.0".to_string()],
                    ["9990.0".to_string(), "0.5".to_string()],
                ],
                asks: vec![
                    ["10100.0".to_string(), "2.0".to_string()],
                    ["10200.0".to_string(), "1.5".to_string()],
                ],
            },
        };

        // Update the order book
        order_book.update(stream_data);

        // Get the full order book
        let (bids, asks) = order_book.get_full_book();

        // Verify bids
        let bids = bids.unwrap();
        assert_eq!(bids.len(), 2);
        assert_eq!(bids[0].price, 9990.0);
        assert_eq!(bids[0].qty, 0.5);
        assert_eq!(bids[1].price, 10000.0);
        assert_eq!(bids[1].qty, 1.0);

        // Verify asks
        let asks = asks.unwrap();
        assert_eq!(asks.len(), 2);
        assert_eq!(asks[0].price, 10100.0);
        assert_eq!(asks[0].qty, 2.0);
        assert_eq!(asks[1].price, 10200.0);
        assert_eq!(asks[1].qty, 1.5);
    }

    // Test the case when the order book is updated with empty data (no bids, no asks)
    #[test]
    fn test_order_book_empty_after_update() {
        let mut order_book = OrderBook::new();

        // Empty stream data (no bids, no asks)
        let stream_data = StreamData {
            stream: "orderBookUpdate".to_string(),
            data: DepthData {
                last_update_id: 1,
                bids: vec![], // No bids
                asks: vec![], // No asks
            },
        };

        // Update the order book with empty data
        order_book.update(stream_data);

        // Verify that get_top() returns None for an empty book
        assert!(order_book.get_top().is_none());

        // Verify that get_full_book() returns None for both bids and asks
        let (bids, asks) = order_book.get_full_book();
        assert!(bids.is_none());
        assert!(asks.is_none());
    }
}