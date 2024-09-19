use tonic::{Request, Response, Status};
use crate::adapters::proto::trade_history_proto::trade_history_proto_service_server::{TradeHistoryProtoService, TradeHistoryProtoServiceServer};
use crate::adapters::proto::trade_history_proto::{Empty, TradeSd as ProtoTradeSD, AverageVolume, TotalVolume};
use crate::domain::entities::trade::{TradeData, TradeSD};
use crate::domain::services::trade_history_service::TradeHistoryService;
use std::sync::Arc;

pub mod trade_history_proto {
    tonic::include_proto!("trade_history_proto"); // gRPC generated code
}

pub struct MyTradeHistoryService {
    trade_history_service: Arc<TradeHistoryService>,
}

impl MyTradeHistoryService {
    pub fn new(trade_history_service: Arc<TradeHistoryService>) -> Self {
        Self { trade_history_service }
    }
}

#[tonic::async_trait]
impl TradeHistoryProtoService for MyTradeHistoryService {
    async fn add_trade(
        &self,
        request: Request<ProtoTradeSD>,
    ) -> Result<Response<Empty>, Status> {
        let trade_sd = request.into_inner();

        if let Some(data) = trade_sd.data {
            // Map ProtoTradeSD to TradeSD
            let trade = TradeSD {
                stream: trade_sd.stream,
                data: TradeData {
                    event_type: data.event_type,
                    event_time: data.event_time,
                    symbol: data.symbol,
                    trade_id: data.trade_id,
                    price: data.price,
                    quantity: data.quantity,
                    trade_time: data.trade_time,
                    is_buyer_market_maker: data.is_buyer_market_maker,
                    ignore: data.ignore,
                },
            };

            // Add trade to history
            self.trade_history_service.add_trade(trade).await;
            Ok(Response::new(Empty {}))
        } else {
            Err(Status::invalid_argument("Trade data is missing"))
        }
    }

    async fn get_average_volume_per_trade(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<AverageVolume>, Status> {
        let average_volume = self.trade_history_service.average_volume_per_trade().await;
        Ok(Response::new(AverageVolume {
            average_volume,
        }))
    }

    async fn get_total_volume(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<TotalVolume>, Status> {
        let total_volume = self.trade_history_service.total_volume().await;
        Ok(Response::new(TotalVolume {
            total_volume,
        }))
    }
}

// Function to create the gRPC service with the real implementation
pub fn create_trade_history_service(
    trade_history_service: Arc<TradeHistoryService>,
) -> TradeHistoryProtoServiceServer<MyTradeHistoryService> {
    TradeHistoryProtoServiceServer::new(MyTradeHistoryService::new(trade_history_service))
}