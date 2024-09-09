use warp::Filter;
use crate::core::order_book_service::OrderBookService;

pub fn create_rest_api() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("orderbook" / "top")
        .and_then(move || async move {
            let service = OrderBookService;
            if let Some(order_book_top) = service.get_top_of_book().await {
                Ok(warp::reply::json(&serde_json::json!({
                    "best_bid_price": order_book_top.best_bid_price,
                    "best_bid_qty": order_book_top.best_bid_qty,
                    "best_ask_price": order_book_top.best_ask_price,
                    "best_ask_qty": order_book_top.best_ask_qty,
                }))) as Result<_, warp::Rejection>
            } else {
                Ok(warp::reply::json(&serde_json::json!({"error": "Order book is empty"}))) as Result<_, warp::Rejection>
            }
        })
}