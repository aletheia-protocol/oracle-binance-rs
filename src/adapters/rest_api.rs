use warp::Filter;
use crate::core::order_book_service::OrderBookServiceTrait;
use crate::core::order_book_service::OrderBookService;
use crate::domain::order_book::OrderBookEntry;

pub fn create_rest_api() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let orderbook_top = warp::path!("orderbook" / "top")
        .and_then(move || async move {
            let service = OrderBookService;
            if let Some(order_book_top) = service.get_top_of_book().await {
                Ok(warp::reply::json(&serde_json::json!({
                    "best_bid_price": order_book_top.best_bid.price,
                    "best_bid_qty": order_book_top.best_bid.qty,
                    "best_ask_price": order_book_top.best_ask.price,
                    "best_ask_qty": order_book_top.best_ask.qty,
                }))) as Result<_, warp::Rejection>
            } else {
                Ok(warp::reply::json(&serde_json::json!({"error": "Order book is empty"}))) as Result<_, warp::Rejection>
            }
        });

    let orderbook_full = warp::path!("orderbook" / "full")
        .and_then(move || async move {
            let service = OrderBookService;
            let (bids, asks) = service.get_full_book().await;

            Ok(warp::reply::json(&serde_json::json!({
                "bids": bids.unwrap_or_default().iter().map(|entry: &OrderBookEntry| {
                    serde_json::json!({
                        "price": entry.price,
                        "qty": entry.qty
                    })
                }).collect::<Vec<_>>(),
                "asks": asks.unwrap_or_default().iter().map(|entry: &OrderBookEntry| {
                    serde_json::json!({
                        "price": entry.price,
                        "qty": entry.qty
                    })
                }).collect::<Vec<_>>()
            }))) as Result<_, warp::Rejection>
        });

    // Combine the routes
    orderbook_top.or(orderbook_full)
}