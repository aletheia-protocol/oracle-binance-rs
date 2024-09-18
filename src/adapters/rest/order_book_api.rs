use warp::{Filter, Rejection};
use warp::reply::Json;
use crate::adapters::rest::service_error::ServiceError;
use crate::domain::services::order_book_service::OrderBookServiceTrait;
use crate::domain::services::order_book_service::OrderBookService;
use crate::domain::entities::order_book::OrderBookEntry;

pub fn create_order_book_api() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let orderbook_top = warp::path!("orderbook" / "top")
        .and_then(move || async move {
            let service = OrderBookService;
            match service.get_top_of_book().await {
                Some(order_book_top) => {
                    Ok(warp::reply::json(&serde_json::json!({
                        "best_bid_price": order_book_top.best_bid.price,
                        "best_bid_qty": order_book_top.best_bid.qty,
                        "best_ask_price": order_book_top.best_ask.price,
                        "best_ask_qty": order_book_top.best_ask.qty,
                    }))) as Result<Json, Rejection>
                }
                None => {
                    Err(warp::reject::custom(ServiceError::EmptyOrderBook))
                }
            }
/*            if let Some(order_book_top) = service.get_top_of_book().await {
                Ok(warp::reply::json(&serde_json::json!({
                    "best_bid_price": order_book_top.best_bid.price,
                    "best_bid_qty": order_book_top.best_bid.qty,
                    "best_ask_price": order_book_top.best_ask.price,
                    "best_ask_qty": order_book_top.best_ask.qty,
                }))) as Result<_, warp::Rejection>
            } else {
                Ok(warp::reply::json(&serde_json::json!({"error": "Order book is empty"}))) as Result<_, warp::Rejection>
            }*/
        });

    let orderbook_full = warp::path!("orderbook" / "full")
        .and_then(move || async move {
            let service = OrderBookService;
            match service.get_full_book().await{
                Some(full_book) => {
                    Ok(warp::reply::json(&serde_json::json!({
                        "bids": full_book.bids.unwrap_or_default().iter().map(|entry: &OrderBookEntry| {
                            serde_json::json!({
                                "price": entry.price,
                                "qty": entry.qty
                            })
                        }).collect::<Vec<_>>(),
                        "asks": full_book.asks.unwrap_or_default().iter().map(|entry: &OrderBookEntry| {
                             serde_json::json!({
                                "price": entry.price,
                                "qty": entry.qty
                            })
                        }).collect::<Vec<_>>()
                    }))) as Result<_, warp::Rejection>
                }
                None => {
                    Err(warp::reject::custom(ServiceError::EmptyOrderBook))
                }
            }


/*            Ok(warp::reply::json(&serde_json::json!({
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
            }))) as Result<_, warp::Rejection>*/
        });

    // Combine the routes
    orderbook_top.or(orderbook_full)
}