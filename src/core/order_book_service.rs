use std::sync::Arc;
use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use crate::domain::order_book::OrderBook;
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

    pub async fn get_top_of_book(&self) {
        let book = ORDER_BOOK.lock().await;
        book.print_top_of_book();
    }
}