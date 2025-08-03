use std::env;

pub struct Config {
    pub solana_rpc_ws: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();
        let solana_rpc_ws = env::var("SOLANA_RPC_WS").expect("SOLANA_RPC_WS must be set");
        let server_port = env::var("SERVER_PORT").unwrap_or("8765".to_string()).parse().unwrap();
        Config { solana_rpc_ws, server_port }
    }
}
