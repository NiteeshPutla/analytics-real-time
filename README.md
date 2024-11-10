# Real-Time Analytics Engine

This project is a implementation for a real-time analytics engine using Rust. It leverages AVL tree algorithms for efficiently processing and aggregating time-series data, including features for real-time data analysis and anomaly detection.

## Features

- **Custom AVL Tree Implementation**: Efficiently manages time-series data for quick insertion and lookup.
- **Real-Time Data Processing**: Supports real-time insertion and anomaly detection for incoming data streams.
- **Multi-threaded Architecture**: Uses multi-threading for improved performance and throughput.

## Getting Started

### Prerequisites

- **Rust & Cargo**: Ensure Rust and Cargo are installed on your system. You can install them using:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh