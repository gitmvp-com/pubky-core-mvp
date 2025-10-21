# Pubky Core MVP

> A simplified implementation of pubky-core demonstrating public-key based key-value storage

## Overview

This is a minimal viable product (MVP) version of [pubky-core](https://github.com/pubky/pubky-core), focusing on the core functionality:

- **Public key based authentication** using ed25519
- **Key-value storage** with PUT/GET/DELETE operations
- **Simple HTTP server** for storage access

## Features

✅ Create keypairs (ed25519)
✅ Store and retrieve data using public keys
✅ HTTP API for storage operations
✅ In-memory storage backend

## Project Structure

```
pubky-core-mvp/
├── Cargo.toml           # Workspace configuration
├── common/              # Shared types and crypto
│   └── src/
│       └── lib.rs       # Keypair, PublicKey, Signature
├── server/              # HTTP server for storage
│   └── src/
│       ├── main.rs      # Server entry point
│       ├── storage.rs   # In-memory storage
│       └── routes.rs    # HTTP routes
└── examples/
    └── basic_usage.rs   # Example usage
```

## Quick Start

### 1. Run the server

```bash
cargo run --bin server
```

The server will start on `http://127.0.0.1:3000`

### 2. Run the example

In a separate terminal:

```bash
cargo run --example basic_usage
```

## Usage Example

```rust
use pubky_common::Keypair;

// Create a new keypair
let keypair = Keypair::random();
let public_key = keypair.public_key();

println!("Public Key: {}", public_key);

// Store data
// PUT http://localhost:3000/{public_key}/data/hello.txt
// Body: "Hello, Pubky!"

// Retrieve data
// GET http://localhost:3000/{public_key}/data/hello.txt
```

## API Endpoints

### PUT /{public_key}/{path}

Store data at the specified path for a public key.

**Example:**
```bash
curl -X PUT http://localhost:3000/abc123.../my-app/data.txt \
  -d "Hello World"
```

### GET /{public_key}/{path}

Retrieve data from the specified path.

**Example:**
```bash
curl http://localhost:3000/abc123.../my-app/data.txt
```

### DELETE /{public_key}/{path}

Delete data at the specified path.

**Example:**
```bash
curl -X DELETE http://localhost:3000/abc123.../my-app/data.txt
```

### GET /{public_key}/{path}/ (List)

List all keys under a path prefix.

**Example:**
```bash
curl http://localhost:3000/abc123.../my-app/
```

## Key Differences from pubky-core

| Feature | pubky-core | This MVP |
|---------|------------|----------|
| Storage Backend | LMDB (persistent) | In-memory HashMap |
| DHT Integration | Pkarr/Mainline DHT | None |
| TLS Support | Yes (Pubky TLS) | No (HTTP only) |
| Authentication | Session cookies + tokens | Public key in URL |
| Authorization | Capabilities-based | None (simplified) |
| WebDAV | Yes | No |
| Rate Limiting | Yes | No |
| Multiple Storage | GCS, Memory, FS | Memory only |

## Dependencies

This MVP uses the same core cryptographic dependencies as pubky-core:

- `ed25519-dalek` (v2.1.1) - Ed25519 signatures
- `rand` (v0.9.0) - Random number generation
- `base32` (v0.5.1) - Base32 encoding for keys
- `axum` (v0.8.1) - HTTP server framework
- `tokio` (v1.43.0) - Async runtime

## Building

```bash
# Build all packages
cargo build

# Build with release optimizations
cargo build --release

# Run tests
cargo test
```

## What's Next?

To extend this MVP towards the full pubky-core functionality:

1. **Persistent storage** - Replace HashMap with LMDB (`heed` crate)
2. **DHT integration** - Add Pkarr for public key DNS
3. **Session management** - Implement cookie-based sessions
4. **TLS support** - Add Pubky TLS for secure connections
5. **Authorization** - Implement capabilities-based access control
6. **Signup flow** - Add homeserver signup/signin

## License

MIT

## Original Project

Based on [pubky-core](https://github.com/pubky/pubky-core) by Synonym.
