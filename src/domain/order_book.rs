use std::collections::BTreeMap;
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use crate::domain::stream_data::StreamData;

pub struct OrderBook {
    pub bids: BTreeMap<OrderedFloat<f64>, f64>,
    pub asks: BTreeMap<OrderedFloat<f64>, f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderBookEntry {
    pub price: f64,
    pub qty: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderBookTop {
    pub best_bid: OrderBookEntry,
    pub best_ask: OrderBookEntry,
}

impl OrderBook {
    pub fn new() -> Self {
        OrderBook {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

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

    pub fn print_top_of_book(&self) {
        if let Some((&best_bid_price, &best_bid_qty)) = self.bids.iter().next_back() {
            log::info!("Best Bid: {} @ {}", best_bid_qty, best_bid_price);
        }
        if let Some((&best_ask_price, &best_ask_qty)) = self.asks.iter().next() {
            log::info!("Best Ask: {} @ {}", best_ask_qty, best_ask_price);
        }
    }

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
            _ => None, // Empty book
        }
    }

    // Updated method to return the full order book (both bids and asks) as Option<Vec<OrderBookEntry>>
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