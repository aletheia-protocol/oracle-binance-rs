#![allow(dead_code)]
use std::sync::Arc;
use async_trait::async_trait;
use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use crate::domain::order_book::{OrderBook, OrderBookEntry, OrderBookTop};
use crate::domain::order_book_sd::OrderBookSD;

pub static ORDER_BOOK: Lazy<Arc<Mutex<OrderBook>>> = Lazy::new(|| {
    Arc::new(Mutex::new(OrderBook::new()))
});

#[async_trait]
pub trait OrderBookServiceTrait: Send + Sync {
    async fn update_order_book(&self, update: OrderBookSD);
    async fn print_top_of_book(&self);
    async fn get_top_of_book(&self) -> Option<OrderBookTop>;
    async fn get_full_book(&self) -> (Option<Vec<OrderBookEntry>>, Option<Vec<OrderBookEntry>>);
}

pub struct OrderBookService;

#[async_trait]
impl OrderBookServiceTrait for OrderBookService {

    async fn update_order_book(&self, update: OrderBookSD) {
        let mut book = ORDER_BOOK.lock().await;
        book.update(update);
    }

    async fn print_top_of_book(&self) {
        let book = ORDER_BOOK.lock().await;
        book.print_top_of_book();
    }

    async fn get_top_of_book(&self) -> Option<OrderBookTop> {
        let book = ORDER_BOOK.lock().await;
        book.get_top()
    }

    async fn get_full_book(&self) -> (Option<Vec<OrderBookEntry>>, Option<Vec<OrderBookEntry>>) {
        let book = ORDER_BOOK.lock().await;
        book.get_full_book()
    }
}