# Running and Testing Instructions

## Prerequisites

Before running the service, ensure you have the following installed:
- Rust 1.70 or higher (install from rustup.rs)
- Cargo (comes with Rust)
- Network access to Solana RPC endpoints
- A text editor for configuration files

## Environment Setup

1. Navigate to the project directory
2. Create a .env file in the project root
3. Add the following environment variables to the .env file:
   - SERVER_PORT: The port for the WebSocket server (default: 8080)
   - SOLANA_RPC_WS: Solana RPC WebSocket URL (default: wss://api.mainnet-beta.solana.com)
   - RUST_LOG: Logging level (default: info)

## Building the Project

1. Open a terminal in the project directory
2. Run the build command for release mode -> cargo build --release
3. Wait for the compilation to complete
4. Verify the executable was created in the target/release directory

## Running the Service

### Development Mode
1. Use the development run command -> cargo run
2. Monitor the console output for startup messages
3. Look for connection confirmation messages
4. Verify the WebSocket server is listening on the configured port

### Production Mode
1. Use the production run command -> cargo run --release
2. The service will start with optimized performance
3. Monitor system resources during operation
4. Check logs for any startup issues

## Service Verification

### Connection Status
1. Look for "Connected to Solana RPC" message
2. Verify "Subscribed to Pump.fun contract" appears
3. Confirm "WebSocket Server running" message
4. Check that the service is waiting for connections

### Log Monitoring
1. Set RUST_LOG to debug for detailed logging
2. Monitor for connection attempts and failures
3. Watch for message processing logs
4. Check for any error messages or warnings

## Testing the WebSocket Connection

### Browser Testing
1. Open browser developer tools
2. Navigate to the console tab
3. Create a WebSocket connection to the service
4. Set up event handlers for connection, messages, and errors
5. Monitor incoming messages
6. Test connection closure and reconnection
