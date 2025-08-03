# Pump.fun WebSocket Service

A high-performance WebSocket service that monitors Pump.fun token creation events on the Solana blockchain and provides real-time structured data to connected clients.

## 🚀 Features

- **Real-time Monitoring**: Subscribes to Pump.fun program account changes on Solana
- **Structured Data Output**: Transforms raw blockchain data into clean, structured JSON events
- **WebSocket Server**: Provides real-time data streaming to multiple clients
- **Automatic Reconnection**: Handles connection drops and automatically reconnects to Solana RPC
- **High Performance**: Built with Rust and Tokio for optimal performance
- **Graceful Shutdown**: Proper cleanup and resource management

## 📋 Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- Solana RPC endpoint (WebSocket URL)
- Network access to Solana RPC

## 🛠️ Installation

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd apeing_ws_service
   ```

2. **Build the project**
   ```bash
   cargo build --release
   ```

3. **Set up environment variables**
   ```bash
   # Create .env file
   cp .env.example .env
   
   # Edit .env with your configuration
   SERVER_PORT=8080
   SOLANA_RPC_WS=wss://api.mainnet-beta.solana.com
   RUST_LOG=info
   ```

## ⚙️ Configuration

The service uses environment variables for configuration:

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `SERVER_PORT` | WebSocket server port | `8080` | No |
| `SOLANA_RPC_WS` | Solana RPC WebSocket URL | `wss://api.mainnet-beta.solana.com` | No |
| `RUST_LOG` | Logging level | `info` | No |

## 🚀 Running the Service

### Development Mode
```bash
cargo run
```

### Production Mode
```bash
cargo run --release
```

## 📡 WebSocket API

### Connection
Connect to the WebSocket server:
```
ws://localhost:8080
```

### Welcome Message
Upon connection, you'll receive a welcome message:
```json
{
  "type": "connection_established",
  "connection_id": 1,
  "message": "Connected to Pump.fun WebSocket Service"
}
```

### Event Format
The service sends structured token creation events in the following format:

```json
{
  "event_type": "token_created",
  "timestamp": "2024-01-15T10:30:45Z",
  "transaction_signature": "5x7K8...",
  "token": {
    "mint_address": "ABC123...",
    "name": "MyToken",
    "symbol": "MTK",
    "creator": "DEF456...",
    "supply": 1000000000,
    "decimals": 6
  },
  "pump_data": {
    "bonding_curve": "GHI789...",
    "virtual_sol_reserves": 30000000000,
    "virtual_token_reserves": 1073000000000000
  }
}
```

## 🔧 Architecture

### Components

1. **Solana Client** (`src/solana_client.rs`)
   - Manages WebSocket connection to Solana RPC
   - Subscribes to Pump.fun program account changes
   - Handles automatic reconnection

2. **Event Parser** (`src/event_parser.rs`)
   - Parses raw Solana RPC notifications
   - Extracts relevant token data
   - Transforms data into structured format

3. **WebSocket Server** (`src/ws_server.rs`)
   - Accepts client connections
   - Broadcasts events to all connected clients
   - Manages connection lifecycle

4. **Configuration** (`src/config.rs`)
   - Loads environment variables
   - Provides configuration validation

### Data Flow

```
Solana RPC → Solana Client → Event Parser → WebSocket Server → Clients
```

## 🧪 Testing

### Unit Tests
```bash
cargo test
```

### Integration Tests
```bash
cargo test --test integration_tests
```

### Manual Testing
1. Start the service
2. Connect a WebSocket client to `ws://localhost:8080`
3. Monitor for incoming events

## 📊 Monitoring

The service provides several monitoring capabilities:

- **Connection Count**: Track active WebSocket connections
- **Message Count**: Monitor messages sent per connection
- **Logging**: Comprehensive logging with configurable levels
- **Error Handling**: Graceful error handling and recovery

## 🔍 Troubleshooting

### Common Issues

1. **Connection Failed**
   - Verify Solana RPC URL is correct
   - Check network connectivity
   - Ensure RPC endpoint supports WebSocket

2. **No Events Received**
   - Verify Pump.fun program ID is correct
   - Check if there are active token creations
   - Review logs for subscription errors
   - check the server port provided in .env file

3. **High Memory Usage**
   - Monitor connection count
   - Check for memory leaks in long-running connections
   - Consider implementing connection limits

### Log Levels

Set `RUST_LOG` environment variable:
- `error`: Only errors
- `warn`: Warnings and errors
- `info`: General information (default)
- `debug`: Detailed debugging information
- `trace`: Very detailed tracing

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request


## 🙏 Acknowledgments

- Solana Labs for the blockchain infrastructure
- Pump.fun team for the token creation platform
- Rust community for excellent async libraries

## 📞 Support

For support and questions:
- Create an issue in the repository
- Check the troubleshooting section
- Review the logs for error details

## 🔄 Changelog

### v1.0.0
- Initial release
- WebSocket server implementation
- Solana RPC integration
- Event parsing and transformation
- Automatic reconnection handling 