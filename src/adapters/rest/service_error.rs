use std::fmt;
use warp::reject::Reject;

#[derive(Debug)]
pub enum ServiceError {
    EmptyOrderBook,
//    OrderBookAccessError,
//    TradeDataError,
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceError::EmptyOrderBook => write!(f, "The order book is empty"),
//            ServiceError::OrderBookAccessError => write!(f, "Could not access the order book"),
//            ServiceError::TradeDataError => write!(f, "Trade data error"),
        }
    }
}

impl Reject for ServiceError {}