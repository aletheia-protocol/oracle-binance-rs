use binance_spot_connector_rust::{
    market_stream::partial_depth::PartialDepthStream,
    tokio_tungstenite::BinanceWebSocketClient,
};
use env_logger::Builder;
use futures_util::StreamExt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct DepthData {
    #[serde(rename = "lastUpdateId")]
    last_update_id: u64,
    bids: Vec<[String; 2]>,
    asks: Vec<[String; 2]>,
}

#[derive(Debug, Deserialize)]
struct StreamData {
    //#[serde(rename = "stream")]
    stream: String,
    data: DepthData,
}

#[tokio::main]
async fn main() {
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();
    // Establish connection
    let (mut conn, _) = BinanceWebSocketClient::connect_async_default()
        .await
        .expect("Failed to connect");
    // Subscribe to streams
    conn.subscribe(vec![
        &PartialDepthStream::from_100ms("BTCFDUSD",10).into()
    ])
        .await;
    // Read messages
    while let Some(message) = conn.as_mut().next().await {
        match message {
            Ok(message) => {
                let binary_data = message.into_data();
                let data = std::str::from_utf8(&binary_data).expect("Failed to parse message");
                //let wrapped_data = format!("r#{}#",data);
                println!("DATA: {}",data);
                if !data.contains(":null")
                {
                    let result: StreamData = serde_json::from_str(format!(r#"{}"#,data).trim()).unwrap();
                    log::info!("#{:?}",result);
                }
                else {
                    log::info!("Empty row: {}",data);
                }
                //log::info!("{:?}", data);
                //let json_str = "r#"+data+"#";
                //println!("JSON_STR: {}",json_str);
                //let result: StreamData = serde_json::from_slice(&binary_data).expect("Failed to parse message");
            }
            Err(_) => break,
        }
    }
    // Disconnect
    conn.close().await.expect("Failed to disconnect");
}