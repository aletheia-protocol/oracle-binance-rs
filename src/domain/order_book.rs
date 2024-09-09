use std::collections::BTreeMap;
use ordered_float::OrderedFloat;
use crate::domain::stream_data::StreamData;

pub struct OrderBook {
    pub bids: BTreeMap<OrderedFloat<f64>, f64>, // zamówienia kupna (OrderedFloat<f64> -> ilość)
    pub asks: BTreeMap<OrderedFloat<f64>, f64>, // zamówienia sprzedaży (OrderedFloat<f64> -> ilość)
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
                self.asks.remove(&OrderedFloat(price)); // Usuń ofertę sprzedaży
            } else {
                self.asks.insert(OrderedFloat(price), qty); // Dodaj lub zaktualizuj ofertę sprzedaży
            }
        }
    }

    // Wyświetlenie topowych ofert kupna i sprzedaży
    pub fn print_top_of_book(&self) {
        if let Some((&best_bid_price, &best_bid_qty)) = self.bids.iter().next_back() {
            println!("Best Bid: {} @ {}", best_bid_qty, best_bid_price);
        }
        if let Some((&best_ask_price, &best_ask_qty)) = self.asks.iter().next() {
            println!("Best Ask: {} @ {}", best_ask_qty, best_ask_price);
        }
    }
}