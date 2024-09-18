use tonic::{Request, Response, Status};
use adapters::proto::book_ticker_proto::book_ticker_proto_service_server::{BookTickerProtoService, BookTickerProtoServiceServer};
use adapters::proto::book_ticker_proto::{Empty, TickerData, MidPrice, MidWeightedPrice};
use std::sync::Arc;
use crate::adapters;
use crate::core::book_ticker_service::BookTickerServiceTrait; // Zaimportuj swój serwis

pub mod book_ticker_proto {
    tonic::include_proto!("book_ticker_proto"); // Wygenerowany kod gRPC
}

pub struct MyBookTickerService {
    book_ticker_service: Arc<dyn BookTickerServiceTrait + Send + Sync>, // Użycie prawdziwej usługi
}

impl MyBookTickerService {
    pub fn new(book_ticker_service: Arc<dyn BookTickerServiceTrait + Send + Sync>) -> Self {
        Self { book_ticker_service }
    }
}

#[tonic::async_trait]
impl BookTickerProtoService for MyBookTickerService {
    async fn get_ticker_data(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<TickerData>, Status> {
        let ticker_data = self.book_ticker_service.get_ticker_data().await;
        let response = TickerData {
            symbol: ticker_data.symbol,
            update_id: ticker_data.update_id as i32,
            best_bid_price: ticker_data.best_bid_price,
            best_bid_qty: ticker_data.best_bid_qty,
            best_ask_price: ticker_data.best_ask_price,
            best_ask_qty: ticker_data.best_ask_qty,
        };
        Ok(Response::new(response))
    }

    async fn get_mid_price(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<MidPrice>, Status> {
        let mid_price = self.book_ticker_service.mid_price().await;
        let response = MidPrice { mid_price };
        Ok(Response::new(response))
    }

    async fn get_mid_weighted_price(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<MidWeightedPrice>, Status> {
        let mid_weighted_price = self.book_ticker_service.mid_weighted_price().await;
        let response = MidWeightedPrice { mid_weighted_price };
        Ok(Response::new(response))
    }
}

// Funkcja do tworzenia serwisu gRPC z prawdziwą implementacją
pub fn create_book_ticker_service(
    book_ticker_service: Arc<dyn BookTickerServiceTrait + Send + Sync>
) -> BookTickerProtoServiceServer<MyBookTickerService> {
    BookTickerProtoServiceServer::new(MyBookTickerService::new(book_ticker_service))
}