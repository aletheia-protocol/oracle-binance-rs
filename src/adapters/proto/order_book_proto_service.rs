use tonic::{Request, Response, Status};
use crate::adapters::proto::order_book_proto::order_book_proto_service_server::{OrderBookProtoService, OrderBookProtoServiceServer};
use crate::adapters::proto::order_book_proto::{Empty, FullOrderBook as ProtoFullOrderBook, OrderBookSd, OrderBookTop as ProtoOrderBookTop};
use crate::domain::entities::order_book::OrderBookSD;
use crate::domain::services::order_book_service::OrderBookServiceTrait;
use std::sync::Arc;

pub mod order_book_proto {
    tonic::include_proto!("order_book_proto"); // Wygenerowany kod gRPC
}

pub struct MyOrderBookService {
    order_book_service: Arc<dyn OrderBookServiceTrait + Send + Sync>, // Korzystanie z rzeczywistej implementacji OrderBookService
}

impl MyOrderBookService {
    pub fn new(order_book_service: Arc<dyn OrderBookServiceTrait + Send + Sync>) -> Self {
        Self { order_book_service }
    }
}

#[tonic::async_trait]
impl OrderBookProtoService for MyOrderBookService {
    async fn update_order_book(
        &self,
        request: Request<OrderBookSd>,
    ) -> Result<Response<Empty>, Status> {
        let order_book_sd = request.into_inner();

        // Zabezpieczamy się przed brakiem wartości w data, używamy tylko raz unwrap
        if let Some(data) = order_book_sd.data {
            // Mapowanie z OrderBookSd (proto) na OrderBookSD (Rust)
            let update = OrderBookSD {
                stream: order_book_sd.stream,
                data: crate::domain::entities::order_book::DepthData {
                    last_update_id: data.last_update_id,
                    bids: data.bids.iter().map(|b| [b.price.clone(), b.qty.clone()]).collect(),
                    asks: data.asks.iter().map(|a| [a.price.clone(), a.qty.clone()]).collect(),
                },
            };

            // Aktualizacja książki zleceń
            self.order_book_service.update_order_book(update).await;

            Ok(Response::new(Empty {}))
        } else {
            Err(Status::invalid_argument("Missing order book data"))
        }
    }

    async fn print_top_of_book(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<Empty>, Status> {
        // Wywołanie metody print_top_of_book z rzeczywistego serwisu
        self.order_book_service.print_top_of_book().await;

        Ok(Response::new(Empty {}))
    }

    async fn get_top_of_book(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<ProtoOrderBookTop>, Status> {
        if let Some(top_of_book) = self.order_book_service.get_top_of_book().await {
            let response = ProtoOrderBookTop {
                best_bid: Some(crate::adapters::proto::order_book_proto::OrderBookEntry {
                    price: top_of_book.best_bid.price,
                    qty: top_of_book.best_bid.qty,
                }),
                best_ask: Some(crate::adapters::proto::order_book_proto::OrderBookEntry {
                    price: top_of_book.best_ask.price,
                    qty: top_of_book.best_ask.qty,
                }),
            };

            Ok(Response::new(response))
        } else {
            Err(Status::not_found("Order book is empty"))
        }
    }

    async fn get_full_book(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<ProtoFullOrderBook>, Status> {
        if let Some(full_book) = self.order_book_service.get_full_book().await {
            let response = ProtoFullOrderBook {
                bids: full_book.bids.unwrap_or_default().iter().map(|b| {
                    crate::adapters::proto::order_book_proto::OrderBookEntry {
                        price: b.price,
                        qty: b.qty,
                    }
                }).collect(),
                asks: full_book.asks.unwrap_or_default().iter().map(|a| {
                    crate::adapters::proto::order_book_proto::OrderBookEntry {
                        price: a.price,
                        qty: a.qty,
                    }
                }).collect(),
            };

            Ok(Response::new(response))
        } else {
            Err(Status::not_found("Full order book is empty"))
        }
    }
}

// Funkcja do tworzenia serwisu gRPC z prawdziwą implementacją
pub fn create_order_book_service(
    order_book_service: Arc<dyn OrderBookServiceTrait + Send + Sync>
) -> OrderBookProtoServiceServer<MyOrderBookService> {
    OrderBookProtoServiceServer::new(MyOrderBookService::new(order_book_service))
}