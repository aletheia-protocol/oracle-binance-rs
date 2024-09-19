# Oracle Binance RS

This project is a Rust-based application that interacts with Binance WebSockets to maintain an order book, track trade history, and provide gRPC and RESTful services for accessing this data.

## Features

### WebSocket Clients
    -   Connects to Binance WebSocket streams to maintain live data.
    -   Handles order book updates and trade history in real-time.
### REST API:
    -   Provides REST endpoints to query the current state of the order book and trade history.
    -   Built with Warp for high-performance HTTP handling.
### gRPC Services:
    -   Exposes gRPC endpoints for fetching real-time book ticker data, order book data, and trade history.
    -   Built using Tonic for gRPC integration.

## Installation

### Prerequisites

	-   Rust: You need to have Rust installed. You can install it from here.
	-   Protobuf Compiler (protoc): Install protoc for generating the gRPC code.

### Build Instructions

1.	Clone the repository:

```shell
git clone https://github.com/your-repository/oracle-binance-rs.git
cd oracle-binance-rs
```
2. Install the required dependencies:
```shell
cargo fetch
```
3. Build the project

```shell
cargo build
```

4. Run the application

```shell
RUST_LOG=info cargo run
```

## Docker Compose

To run this project with the docker compose you have to build the image:

```shell
docker compose up --build
```

## Environment Variables

The application uses environment variables to configure certain aspects:

	- GRPC_PORT: Port for the gRPC service (default: 50051).
	- REST_PORT: Port for the REST API service (default: 8080).

These can be set in your .env file or in docker-compose.yml when using Docker.

## Usage

### REST API

	-   GET /order-book: Fetch the current order book.
	-   GET /book-ticker: Fetch the current book ticker data.
	-   GET /trade-history: Fetch the trade history.

### gRPC Services

The application exposes the following gRPC methods:

	-   BookTickerProtoService:
	-   GetTickerData: Fetch the ticker data.
	-   GetMidPrice: Get the mid-price.
	-   GetMidWeightedPrice: Get the weighted mid-price.
	-   OrderBookProtoService:
	-   UpdateOrderBook: Update the order book.
	-   PrintTopOfBook: Print the top of the book.
	-   GetTopOfBook: Get the top of the order book.
	-   TradeHistoryProtoService:
	-   AddTrade: Add a new trade.
	-   GetAverageVolumePerTrade: Get the average volume of trades.
	-   GetTotalVolume: Get the total trade volume in the last 60 seconds.

## License

This project is licensed under the MIT License.
