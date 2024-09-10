use std::sync::Arc;
use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use crate::domain::order_book::{OrderBook, OrderBookEntry, OrderBookTop};
use crate::domain::stream_data::StreamData;

pub static ORDER_BOOK: Lazy<Arc<Mutex<OrderBook>>> = Lazy::new(|| {
    Arc::new(Mutex::new(OrderBook::new()))
});


pub struct OrderBookService;

impl OrderBookService {

    pub async fn update_order_book(&self, update: StreamData) {
        let mut book = ORDER_BOOK.lock().await;
        book.update(update);
    }

    pub async fn print_top_of_book(&self) {
        let book = ORDER_BOOK.lock().await;
        book.print_top_of_book();
    }

    pub async fn get_top_of_book(&self) -> Option<OrderBookTop> {
        let book = ORDER_BOOK.lock().await;
        book.get_top()
    }

    pub async fn get_full_book(&self) -> (Option<Vec<OrderBookEntry>>, Option<Vec<OrderBookEntry>>) {
        let book = ORDER_BOOK.lock().await;
        book.get_full_book()
    }
}