use std::sync::Arc;
use warp::Filter;
use crate::domain::services::trade_history_service::TradeHistoryService;

pub fn create_trade_history_rest_api() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // Route to get the average volume per trade
    let average_volume = warp::path!("tradehistory" / "average_volume")
        .and_then(move || async move {
            let service = Arc::new(TradeHistoryService);
            let avg_volume = service.average_volume_per_trade().await;

            Ok(warp::reply::json(&serde_json::json!({
                "average_volume_per_trade": avg_volume
            }))) as Result<_, warp::Rejection>
        });

    // Route to get the total volume of trades in the rolling window
    let total_volume = warp::path!("tradehistory" / "total_volume")
        .and_then(move || async move {
            let service = TradeHistoryService;
            let total_volume = service.total_volume().await;

            Ok(warp::reply::json(&serde_json::json!({
                "total_volume": total_volume
            }))) as Result<_, warp::Rejection>
        });

    // Combine both routes
    average_volume
        .or(total_volume)
}