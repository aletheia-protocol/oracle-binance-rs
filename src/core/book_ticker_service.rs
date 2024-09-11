use std::sync::Arc;
use async_trait::async_trait;
use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};
use crate::domain::book_ticker::{BookTickerData, BookTickerSD};

// Singleton for BookTickerData shared across the application
pub static BOOK_TICKER: Lazy<Arc<Mutex<BookTickerData>>> = Lazy::new(|| {
    Arc::new(Mutex::new(BookTickerData::default()))
});

// Trait for the BookTickerService that defines the methods
#[async_trait]
pub trait BookTickerServiceTrait: Send + Sync {
    async fn update_ticker(&self, update: BookTickerSD);
    async fn print_ticker(&self);
    async fn mid_price(&self) -> f64;
    async fn mid_weighted_price(&self) -> f64;
    async fn get_ticker_data(&self) -> BookTickerData;
}

// Implementation of BookTickerService
pub struct BookTickerService;

#[async_trait]
impl BookTickerServiceTrait for BookTickerService {
    async fn update_ticker(&self, update: BookTickerSD) {
        let mut ticker = BOOK_TICKER.lock().await;
        ticker.u = update.data.u;
        ticker.s = update.data.s;
        ticker.b = update.data.b;
        ticker.B = update.data.B;
        ticker.a = update.data.a;
        ticker.A = update.data.A;
    }

    async fn print_ticker(&self) {
        let ticker = BOOK_TICKER.lock().await;
        ticker.print();
    }

    async fn mid_price(&self) -> f64 {
        let ticker = BOOK_TICKER.lock().await;
        ticker.mid_price()
    }

    async fn mid_weighted_price(&self) -> f64 {
        let ticker = BOOK_TICKER.lock().await;
        ticker.mid_weighted_price()
    }

    async fn get_ticker_data(&self) -> BookTickerData {
        let ticker = BOOK_TICKER.lock().await;
        ticker.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::book_ticker::{BookTickerData, BookTickerSD};
    use super::*;

    #[tokio::test]
    async fn test_mid_price() {
        let service = BookTickerService;

        let data = BookTickerSD {
            stream: "btcfdusd@bookTicker".to_string(),
            data: BookTickerData {
                u: 123,
                s: "BTCUSD".to_string(),
                b: "50000.0".to_string(),
                B: "2.0".to_string(),
                a: "51000.0".to_string(),
                A: "3.0".to_string(),
            }
        };

        service.update_ticker(data).await;

        let mid_price = service.mid_price().await;
        assert_eq!(mid_price, 50500.0); // (50000 + 51000) / 2
    }

    #[tokio::test]
    async fn test_mid_weighted_price() {
        let service = BookTickerService;

        let data = BookTickerSD {
            stream: "btcfdusd@bookTicker".to_string(),
            data: BookTickerData {
                u: 123,
                s: "BTCUSD".to_string(),
                b: "50000.0".to_string(),
                B: "2.0".to_string(),
                a: "51000.0".to_string(),
                A: "3.0".to_string(),
            }
        };

        service.update_ticker(data).await;

        let mid_weighted_price = service.mid_weighted_price().await;
        // Weighted average: (50000 * 2 + 51000 * 3) / (2 + 3) = 50600
        assert_eq!(mid_weighted_price, 50600.0);
    }
}