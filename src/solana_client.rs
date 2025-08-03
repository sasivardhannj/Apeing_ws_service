use tokio_tungstenite::connect_async;
use futures::{SinkExt, StreamExt};
use tokio::sync::broadcast::Sender;
use serde_json::json;
use log::{info, error};
use crate::event_parser;

/// Establishes and maintains a WebSocket connection to Solana RPC
/// Subscribes to pump.fun contract events and broadcasts them to connected clients
pub async fn solana_event_listener(sender: Sender<String>, rpc_url: String) {
    loop {
        // Attempt to establish WebSocket connection to Solana RPC
        match connect_async(&rpc_url).await {
            Ok((ws_stream, _)) => {
                info!("Connected to Solana RPC");
                let (mut write, mut read) = ws_stream.split();

                // Create subscription message for pump.fun program account changes
                // This subscribes to all account changes for the pump.fun contract
                let subscription = json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "method": "programSubscribe",
                    "params": [
                        "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P", // pump.fun program ID
                        {"encoding": "jsonParsed"} // Request parsed JSON data
                    ]
                });

                // Send subscription request to Solana RPC
                if let Err(e) = write.send(tungstenite::Message::Text(subscription.to_string())).await {
                    error!("Subscription error: {:?}", e);
                    continue; // Retry connection on subscription failure
                }

                info!("Subscribed to Pump.fun contract.");

                // Process incoming messages from Solana RPC
                while let Some(msg) = read.next().await {
                    match msg {
                        Ok(tungstenite::Message::Text(txt)) => {
                            // Try to parse the raw message into structured format
                            if let Some(parsed_event) = event_parser::parse_event(&txt) {
                                // Send the structured event to clients
                                let _ = sender.send(parsed_event);
                            } else {
                                // If parsing fails, send the raw message for debugging
                                let _ = sender.send(txt);
                            }
                        }
                        Ok(_) => {
                            // Ignore non-text messages (binary, ping, pong, etc.)
                        },
                        Err(e) => {
                            error!("WebSocket read error: {:?}", e);
                            break; // Exit message loop on read error
                        }
                    }
                }

                error!("Disconnected. Reconnecting...");
            }
            Err(e) => {
                error!("Failed to connect: {:?}", e);
                // Wait 5 seconds before attempting to reconnect
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        }
    }
}