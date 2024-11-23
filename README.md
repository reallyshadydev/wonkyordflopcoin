
---

# Pepecoin Ord Installation and Setup Guide

**ord-pepecoin** is a fork of [verydogelabs/wonky-ord-dogecoin](https://github.com/verydogelabs/wonky-ord-dogecoin), tailored for the Pepecoin blockchain. This guide explains how to install, configure, and run the **ord-pepecoin** indexer and server.

---

## Installation Steps

### 1. **Install Required Dependencies**

Update your system and install the necessary dependencies:

```bash
sudo apt-get update
sudo apt-get install -y build-essential libssl-dev pkg-config curl git
```

---

### 2. **Install Rust and Cargo**

Rust is required to build **ord-pepecoin** from source. Install Rust and Cargo with:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Verify the installation:

```bash
cargo --version
```

---

### 3. **Clone and Build ord-pepecoin**

Clone the repository and build the project:

```bash
git clone https://github.com/PepeEnthusiast/wonky-ord-pepecoin.git
cd wonky-ord-pepecoin
cargo build --release
```

The compiled binary will be available in the `target/release` directory.

---

### 4. **Install and Sync Pepecoin Core**

Ensure your Pepecoin node is installed and fully synced:

- Install Pepecoin Core using the official repository or [Pepecoin Community](https://github.com/PepeEnthusiast/pepecoin-core).
- Configure the `pepecoin.conf` file with the following settings:
  ```plaintext
  rpcuser=your_rpc_username
  rpcpassword=your_rpc_password
  rpcallowip=127.0.0.1
  rpcport=33874
  server=1
  daemon=1
  txindex=1
  ```
- Start the Pepecoin Core node:
  ```bash
  pepecoind
  ```
- Ensure the node is fully synced by checking the blockchain info:
  ```bash
  pepecoin-cli getblockchaininfo
  ```

---

## Running ord-pepecoin Indexer and Server

### Command Line Options

- **`--index-transactions`**: Enables storing transaction data for API queries.
- **`--index-drc20`**: Tracks PRC tokens, balances, tick holders, and lists.
- **`--index-dunes`**: Tracks dune data, similar to Rune or Dune concepts.
- **`--nr-parallel-requests`**: Configures the number of parallel RPC requests to the Pepecoin node. Default is `16`.

---

### 5. **Run ord-pepecoin Indexer**

Run the indexer to process blockchain data:

```bash
./target/release/ord \
    --first-inscription-height=186920 \
    --rpc-url=http://your_rpc_username:your_rpc_password@127.0.0.1:33874/ \
    --data-dir=/root/.local/share/ord \
    --index-transactions \
    --index-drc20 \
    --index-dunes \
    index
```

### Explanation:
- **`--first-inscription-height=186920`**: Starts indexing from block height 186920.
- **`--rpc-url=http://your_rpc_username:your_rpc_password@127.0.0.1:33874/`**: Connects to your Pepecoin Core node.
- **`--data-dir=/root/.local/share/ord`**: Specifies the directory for indexed data.
- **`--index-transactions`**, **`--index-drc20`**, **`--index-dunes`**: Enables indexing for transactions, PRC tokens, and dunes.

---

### 6. **Run ord-pepecoin Server**

After indexing is complete, start the **ord-pepecoin** server:

```bash
./target/release/ord \
    --first-inscription-height=186920 \
    --rpc-url=http://your_rpc_username:your_rpc_password@127.0.0.1:33874/ \
    --data-dir=/root/.local/share/ord \
    --index-transactions \
    --index-drc20 \
    --index-dunes \
    server --http-port=<your_port>
```

Replace `<your_port>` with your desired HTTP port (e.g., `8420`).

---

### Accessing the Server

Once the server is running, you can access it at:

```plaintext
http://127.0.0.1:<your_port>
```

---

## Optional: Redirect Logs

### For Indexing:

```bash
./target/release/ord \
    --first-inscription-height=186920 \
    --rpc-url=http://your_rpc_username:your_rpc_password@127.0.0.1:33874/ \
    --data-dir=/root/.local/share/ord \
    --index-transactions \
    --index-drc20 \
    --index-dunes \
    index > ~/ord-pepecoin-index.log 2>&1
```

### For Server:

```bash
./target/release/ord \
    --first-inscription-height=186920 \
    --rpc-url=http://your_rpc_username:your_rpc_password@127.0.0.1:33874/ \
    --data-dir=/root/.local/share/ord \
    --index-transactions \
    --index-drc20 \
    --index-dunes \
    server --http-port=<your_port> > ~/ord-pepecoin-server.log 2>&1
```

---

## Troubleshooting

### Ensure Pepecoin Core is Fully Synced
Before starting the indexer or server, verify that your Pepecoin node is fully synced:

```bash
pepecoin-cli getblockchaininfo
```

### Clean Indexer Data
If you need to restart from scratch, delete the existing indexer data:

```bash
rm -rf /root/.local/share/ord
```

Then rerun the indexing command.

---

## Running ord-pepecoin in Docker (Optional)

If you prefer Docker, follow these steps:

1. Install Docker and Docker-Compose ([Docker installation guide](https://docs.docker.com/engine/install/ubuntu/)).
2. Clone the **ord-pepecoin** repository and navigate to the directory:
   ```bash
   git clone https://github.com/PepeEnthusiast/wonky-ord-pepecoin.git
   cd wonky-ord-pepecoin
   ```
3. Build and run the Docker container:
   ```bash
   docker-compose build
   docker-compose up -d
   ```
4. View logs:
   ```bash
   docker-compose logs -f --tail 200
   ```

---

