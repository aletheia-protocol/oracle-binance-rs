use std::sync::Arc;
use warp::Filter;
use crate::domain::services::book_ticker_service::{BookTickerServiceTrait, BookTickerService};

pub fn create_book_ticker_rest_api() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // Route to get the full book ticker data
    let book_ticker_data = warp::path!("bookticker" / "data")
        .and_then(move || async move {
            let service = Arc::new(BookTickerService);
            let ticker_data = service.get_ticker_data().await;

            // Return the full book ticker data
            Ok(warp::reply::json(&serde_json::json!({
                "symbol": ticker_data.symbol,
                "update_id": ticker_data.update_id,
                "best_bid_price": ticker_data.best_bid_price,
                "best_bid_qty": ticker_data.best_bid_qty,
                "best_ask_price": ticker_data.best_ask_price,
                "best_ask_qty": ticker_data.best_ask_qty,
            }))) as Result<_, warp::Rejection>
        });

    // Route to get the mid price only
    let book_ticker_midprice = warp::path!("bookticker" / "midprice")
        .and_then(move || async move {
            let service = BookTickerService;
            let mid_price = service.mid_price().await;

            Ok(warp::reply::json(&serde_json::json!({
                "mid_price": mid_price
            }))) as Result<_, warp::Rejection>
        });

    // Route to get the mid-weighted price only
    let book_ticker_midweightedprice = warp::path!("bookticker" / "midweightedprice")
        .and_then(move || async move {
            let service = BookTickerService;
            let mid_weighted_price = service.mid_weighted_price().await;

            Ok(warp::reply::json(&serde_json::json!({
                "mid_weighted_price": mid_weighted_price
            }))) as Result<_, warp::Rejection>
        });

    // Combine all routes
    book_ticker_data
        .or(book_ticker_midprice)
        .or(book_ticker_midweightedprice)
}