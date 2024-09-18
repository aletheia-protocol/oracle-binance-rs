use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradeSD {
    pub stream: String,   // Stream name (e.g., btcfdusd@tradeStream)
    pub data: TradeData, // TradeStream data
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradeData {
    #[serde(rename = "e")]
    pub event_type: String,        // Event type (trade)
    #[serde(rename = "E")]
    pub event_time: u64,           // Event time
    #[serde(rename = "s")]
    pub symbol: String,            // Symbol (e.g., BNBBTC)
    #[serde(rename = "t")]
    pub trade_id: u64,             // Trade ID
    #[serde(rename = "p")]
    pub price: String,             // Price
    #[serde(rename = "q")]
    pub quantity: String,          // Quantity
    #[serde(rename = "T")]
    pub trade_time: u64,           // Trade time
    #[serde(rename = "m")]
    pub is_buyer_market_maker: bool, // Is the buyer the market maker?
    #[serde(rename = "M")]
    pub ignore: bool               // Ignore
}

