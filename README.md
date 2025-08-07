# Solana Fellowship API

A comprehensive REST API for Solana blockchain operations built with Rust and Actix-web. This service provides endpoints for keypair generation, token operations, message signing, and transaction instruction creation.

## Features

- 🔐 **Keypair Management**: Generate Solana keypairs securely
- 💰 **SOL Transfers**: Create SOL transfer instructions
- 🪙 **SPL Token Operations**: Create, mint, and transfer SPL tokens
- ✍️ **Message Signing**: Sign and verify messages with Solana keypairs
- 🏗️ **Transaction Instructions**: Generate serialized transaction instructions for client-side execution

## Table of Contents

- [Installation](#installation)
- [Running the Server](#running-the-server)
- [API Endpoints](#api-endpoints)
- [Request/Response Formats](#requestresponse-formats)
- [Testing](#testing)
- [Project Structure](#project-structure)
- [Contributing](#contributing)

## Installation

### Prerequisites

- Rust (1.70.0 or later)
- Cargo (comes with Rust)

### Clone and Build

```bash
git clone <repository-url>
cd superdevs-assignment
cargo build --release
```

## Running the Server

### Development Mode
```bash
cargo run
```

### Production Mode
```bash
cargo run --release
```

The server will start on `http://localhost:8080`

## API Endpoints

### Health Check
- **GET** `/` - Server health check

### Keypair Operations
- **POST** `/keypair` - Generate a new Solana keypair

### SOL Transfer
- **POST** `/send/sol` - Create SOL transfer instruction

### SPL Token Operations
- **POST** `/token/create` - Create SPL token mint instruction
- **POST** `/token/mint` - Create token minting instruction
- **POST** `/send/token` - Create SPL token transfer instruction

### Message Operations
- **POST** `/message/sign` - Sign a message with a keypair
- **POST** `/message/verify` - Verify a message signature

## Request/Response Formats

### Standard Response Format

All endpoints return responses in this format:

**Success Response:**
```json
{
  "success": true,
  "data": {
    // endpoint-specific data
  }
}
```

**Error Response:**
```json
{
  "success": false,
  "error": "Error description"
}
```

### Endpoint Details

#### 1. Generate Keypair
**POST** `/keypair`

**Request:** No body required

**Response:**
```json
{
  "success": true,
  "data": {
    "pubkey": "base58-encoded-public-key",
    "secret": "base58-encoded-secret-key"
  }
}
```

#### 2. Create SOL Transfer Instruction
**POST** `/send/sol`

**Request:**
```json
{
  "from": "sender-public-key",
  "to": "recipient-public-key",
  "lamports": 1000000
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "program_id": "11111111111111111111111111111112",
    "accounts": ["account1", "account2"],
    "instruction_data": "base58-encoded-instruction-data"
  }
}
```

#### 3. Create SPL Token
**POST** `/token/create`

**Request:**
```json
{
  "mint": "mint-keypair-public-key",
  "mintAuthority": "mint-authority-public-key",
  "freezeAuthority": "freeze-authority-public-key",
  "decimals": 9
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "program_id": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
    "accounts": [
      {
        "pubkey": "account-public-key",
        "isSigner": false
      }
    ],
    "instruction_data": "base58-encoded-instruction-data"
  }
}
```

#### 4. Mint SPL Tokens
**POST** `/token/mint`

**Request:**
```json
{
  "mint": "mint-public-key",
  "destination": "destination-token-account",
  "mintAuthority": "mint-authority-public-key",
  "amount": 1000000
}
```

#### 5. Transfer SPL Tokens
**POST** `/send/token`

**Request:**
```json
{
  "destination": "destination-public-key",
  "mint": "mint-public-key",
  "owner": "token-owner-public-key",
  "amount": 1000000
}
```

#### 6. Sign Message
**POST** `/message/sign`

**Request:**
```json
{
  "message": "Hello, Solana!",
  "secret": "base58-encoded-secret-key"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "signature": "base58-encoded-signature",
    "pubkey": "signer-public-key",
    "message": "Hello, Solana!"
  }
}
```

#### 7. Verify Message Signature
**POST** `/message/verify`

**Request:**
```json
{
  "message": "Hello, Solana!",
  "signature": "base58-encoded-signature",
  "pubkey": "signer-public-key"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "valid": true,
    "message": "Hello, Solana!",
    "pubkey": "signer-public-key"
  }
}
```

## Testing

The project includes comprehensive integration tests written in JavaScript using Jest.

### Prerequisites for Testing
```bash
npm install
```

### Run Tests
```bash
# Run with local server
npm test

# Run with deployed server
HTTP_URL=https://your-deployment-url.com npm test
```

### Test Coverage
- Keypair generation and validation
- SOL transfer instructions
- SPL token creation, minting, and transfers
- Message signing and verification
- Error handling for invalid inputs
- Edge cases and security validations

## Project Structure

```
src/
├── main.rs              # Application entry point and server setup
├── handlers/            # HTTP request handlers
│   ├── mod.rs
│   ├── keypair_handlers.rs    # Keypair generation endpoints
│   ├── message_handlers.rs    # Message signing/verification
│   ├── token_handler.rs       # SPL token operations
│   └── transfer_handler.rs    # SOL/token transfer operations
├── services/            # Business logic layer
│   ├── keypair_services.rs    # Keypair generation logic
│   ├── message_services.rs    # Message signing/verification logic
│   ├── token_services.rs      # SPL token operations
│   └── transfer_service.rs    # Transfer instruction creation
└── models/              # Data structures and types
    ├── mod.rs
    ├── responses.rs           # API response types
    ├── token_model.rs         # Token-related models
    └── transfer_models.rs     # Transfer-related models

tests/
└── test.js              # Integration tests
```

## Dependencies

### Core Dependencies
- **actix-web**: Web framework for building HTTP APIs
- **solana-sdk**: Solana blockchain SDK for Rust
- **solana-client**: Solana RPC client
- **spl-token**: SPL Token program bindings
- **spl-associated-token-account**: Associated Token Account utilities

### Utility Dependencies
- **serde**: Serialization/deserialization framework
- **bs58**: Base58 encoding/decoding
- **base64**: Base64 encoding utilities

## Error Handling

The API provides detailed error messages for common issues:

- **400 Bad Request**: Invalid input parameters, malformed public keys, insufficient amounts
- **500 Internal Server Error**: Server-side errors during instruction creation

Common error scenarios:
- Invalid public key formats
- Missing required fields
- Invalid amounts (negative or zero values)
- Malformed secret keys for message signing

## Security Considerations

1. **Private Key Handling**: The API accepts base58-encoded private keys for message signing but never stores them
2. **Input Validation**: All public keys and parameters are validated before processing
3. **Error Messages**: Errors are descriptive but don't leak sensitive information
4. **Instruction Generation**: The API only generates instructions; actual transaction signing and submission must be done client-side

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass (`cargo test` and `npm test`)
6. Commit your changes (`git commit -m 'Add amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Support

For questions or issues:
1. Check the existing GitHub issues
2. Create a new issue with detailed description
3. Include request/response examples when reporting API issues

---
