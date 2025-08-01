# msredis

[![Crates.io](https://img.shields.io/crates/v/msredis.svg)](https://crates.io/crates/msredis)
[![Documentation](https://docs.rs/msredis/badge.svg)](https://docs.rs/msredis)
[![Build Status](https://github.com/your-github-username/msredis/workflows/CI/badge.svg)](https://github.com/your-github-username/msredis/actions)

`msredis` refer to **Memory Safe redis**, it is a Redis client library written in Rust, designed to provide a simple, efficient, and easy-to-use interface for interacting with Redis servers.

## Features
- Support for basic Redis commands (e.g., `GET`, `SET`, `DEL`).
- Asynchronous interface based on the `tokio` asynchronous runtime.
- Automatic reconnection mechanism to ensure reliable connections.
- Comprehensive error handling and logging support.

## Installation

### Add Dependency
Add the following to your `Cargo.toml` file:
```toml
[dependencies]
msredis = "0.1.0"