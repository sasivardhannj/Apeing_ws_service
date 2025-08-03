use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures::{SinkExt, StreamExt};
use tokio::sync::broadcast::Receiver;
use log::{info, warn, error, debug};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

// Global connection counter for monitoring
static CONNECTION_COUNT: AtomicUsize = AtomicUsize::new(0);

/// Starts the WebSocket server and handles client connections
/// 
/// # Arguments
/// * `port` - The port number to bind the server to
/// * `receiver` - Broadcast receiver for incoming events
pub async fn start_ws_server(port: u16, receiver: Receiver<String>) {
    let addr = format!("0.0.0.0:{}", port);
    
    // Bind to the specified address
    let listener = match TcpListener::bind(&addr).await {
        Ok(listener) => {
            info!("WebSocket Server running on {}", addr);
            listener
        }
        Err(e) => {
            error!("Failed to bind port {}: {}", port, e);
            return;
        }
    };

    info!("Waiting for WebSocket connections...");

    loop {
        // Accept new connections
        match listener.accept().await {
            Ok((stream, addr)) => {
                let connection_id = CONNECTION_COUNT.fetch_add(1, Ordering::SeqCst);
                info!("New connection #{} from {}", connection_id, addr);
                
                // Create a new receiver for this client
                let mut rx = receiver.resubscribe();
                
                // Spawn a new task to handle this client
                tokio::spawn(async move {
                    handle_client_connection(stream, rx, connection_id, addr).await;
                });
            }
            Err(e) => {
                error!("Failed to accept connection: {}", e);
            }
        }
    }
}

/// Handles an individual client WebSocket connection
async fn handle_client_connection(
    stream: tokio::net::TcpStream,
    mut rx: Receiver<String>,
    connection_id: usize,
    addr: std::net::SocketAddr,
) {
    // Accept the WebSocket connection
    let ws_stream = match accept_async(stream).await {
        Ok(ws_stream) => {
            info!("WebSocket connection #{} established from {}", connection_id, addr);
            ws_stream
        }
        Err(e) => {
            error!("Failed to accept WebSocket connection #{} from {}: {}", connection_id, addr, e);
            return;
        }
    };

    let (mut write, _) = ws_stream.split();
    
    // Send welcome message
    let welcome_msg = serde_json::json!({
        "type": "connection_established",
        "connection_id": connection_id,
        "message": "Connected to Pump.fun WebSocket Service"
    });
    
    if let Err(e) = write.send(tungstenite::Message::Text(welcome_msg.to_string())).await {
        warn!("Failed to send welcome message to connection #{}: {}", connection_id, e);
    }

    // Process incoming events and send to client
    let mut message_count = 0u64;
    
    while let Ok(message) = rx.recv().await {
        message_count += 1;
        debug!("Sending message #{} to connection #{}", message_count, connection_id);
        
        match write.send(tungstenite::Message::Text(message.clone())).await {
            Ok(_) => {
                // Message sent successfully
            }
            Err(e) => {
                warn!("Failed to send message to connection #{}: {}", connection_id, e);
                break;
            }
        }
    }

    // Update connection count
    CONNECTION_COUNT.fetch_sub(1, Ordering::SeqCst);
    info!("Connection #{} from {} disconnected. Total messages sent: {}", 
          connection_id, addr, message_count);
}

/// Returns the current number of active connections
pub fn get_active_connections() -> usize {
    CONNECTION_COUNT.load(Ordering::SeqCst)
}