mod config;
mod solana_client;
mod event_parser;
mod ws_server;

use tokio::sync::broadcast;
use tokio::signal;
use log::{info, warn, error};

#[tokio::main]
async fn main() {
    // Initialize logging with better configuration
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    
    info!("Starting Pump.fun WebSocket Service...");
    
    let config = config::Config::from_env();
    info!("Configuration loaded - Server port: {}, Solana RPC: {}", config.server_port, config.solana_rpc_ws);

    // Create broadcast channel for event distribution
    let (sender, _) = broadcast::channel(1000); // Increased buffer size for better performance

    // Spawn Solana event listener task
    let solana_sender = sender.clone();
    let solana_url = config.solana_rpc_ws.clone();
    let solana_handle = tokio::spawn(async move {
        solana_client::solana_event_listener(solana_sender, solana_url).await;
    });



    // Spawn WebSocket server task
    let ws_handle = tokio::spawn(async move {
        ws_server::start_ws_server(config.server_port, sender.subscribe()).await;
    });

    // Wait for shutdown signal
    info!("Service running. Press Ctrl+C to shutdown gracefully...");
    
    match signal::ctrl_c().await {
        Ok(()) => {
            info!("Shutdown signal received. Gracefully shutting down...");
        }
        Err(err) => {
            error!("Unable to listen for shutdown signal: {}", err);
        }
    }

    // Graceful shutdown
    info!("Initiating graceful shutdown...");
    
    // Cancel all tasks
    solana_handle.abort();
    ws_handle.abort();
    
    // Wait for tasks to finish
    let _ = tokio::join!(
        solana_handle,
        ws_handle
    );
    
    info!("Service shutdown complete.");
}