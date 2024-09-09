use std::collections::BTreeMap;
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use crate::domain::stream_data::StreamData;

pub struct OrderBook {
    pub bids: BTreeMap<OrderedFloat<f64>, f64>,
    pub asks: BTreeMap<OrderedFloat<f64>, f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderBookTop {
    pub best_bid_price: f64,
    pub best_bid_qty: f64,
    pub best_ask_price: f64,
    pub best_ask_qty: f64,
}

impl OrderBook {
    pub fn new() -> Self {
        OrderBook {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    pub fn update(&mut self, update: StreamData) {
        for row in update.data.bids {
            let price = row[0].parse::<f64>().unwrap();
            let qty = row[1].parse::<f64>().unwrap();
            if qty == 0.0 {
                self.bids.remove(&OrderedFloat(price));
            } else {
                self.bids.insert(OrderedFloat(price), qty);
            }
        }

        for row in update.data.asks {
            let price = row[0].parse::<f64>().unwrap();
            let qty = row[1].parse::<f64>().unwrap();
            if qty == 0.0 {
                self.asks.remove(&OrderedFloat(price));
            } else {
                self.asks.insert(OrderedFloat(price), qty);
            }
        }
    }

    pub fn print_top_of_book(&self) {
        if let Some((&best_bid_price, &best_bid_qty)) = self.bids.iter().next_back() {
            println!("Best Bid: {} @ {}", best_bid_qty, best_bid_price);
        }
        if let Some((&best_ask_price, &best_ask_qty)) = self.asks.iter().next() {
            println!("Best Ask: {} @ {}", best_ask_qty, best_ask_price);
        }
    }

    pub fn get_top(&self) -> Option<OrderBookTop> {
        let best_bid = self.bids.iter().next_back();
        let best_ask = self.asks.iter().next();

        match (best_bid, best_ask) {
            (Some((&best_bid_price, &best_bid_qty)), Some((&best_ask_price, &best_ask_qty))) => {
                Some(OrderBookTop {
                    best_bid_price: best_bid_price.into_inner(),
                    best_bid_qty,
                    best_ask_price: best_ask_price.into_inner(),
                    best_ask_qty,
                })
            }
            _ => None, // Empty book
        }
    }
}