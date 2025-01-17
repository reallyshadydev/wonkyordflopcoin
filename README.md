Here is the full revised README with the simplified setup instructions:

---

# Wonky Ord Flopcoin

ℹ️ This project is a fork of [apezord/ord-dogecoin](https://github.com/apezord/ord-dogecoin).

## Key Differences

‼️ **DISCLAIMER: OUR CODE MAY STILL HAVE BUGS** ‼️

This project implements actual Flopcoin block rewards for blocks 0 through 144,999. Please review our code in `src/epoch.rs` and let us know of any issues.

---

## Prerequisites

### 1. Install and Sync Flopcoin Core
1. Clone and install Flopcoin Core from the official repository:
   [Flopcoin Core](https://github.com/Flopcoin/Flopcoin)
2. Configure your `flopcoin.conf` file with the following parameters:
   ```conf
   rpcuser=1234
   rpcpassword=1234
   dns=1
   irc=1
   listen=1
   dnsseed=1
   daemon=1
   server=1
   rpcport=32552
   allowip=127.0.0.1
   debug=1
   ```
3. Fully sync your Flopcoin node before proceeding.

### 2. Install Rust
Ensure Rust is installed on your system:
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
After installation, ensure Rust is up to date:
```shell
rustup update
```

---

## Installation and Build Instructions

1. Clone the Wonky Ord repository:
   ```shell
   git clone https://github.com/reallyshadydev/wonkyordflopcoin.git
   cd wonkyordflopcoin
   ```

2. Build the project:
   ```shell
   cargo build --release
   ```

3. Create the data directory:
   ```shell
   mkdir -p /mnt/ord-node/indexer-data
   ```

4. Start the `ord` indexer or server:
   Replace `YOUR_RPC_URL` with your Flopcoin Core RPC URL:

   ### Run as Server
   ```shell
   ./target/release/ord \
       --first-inscription-height=0 \
       --rpc-url=http://1234:1234@127.0.0.1:32552 \
       --data-dir=/mnt/ord-node/indexer-data \
       --index-transactions \
       --index-drc20 \
       --nr-parallel-requests=16 \
       --index-dunes \
       server
   ```

   ### Run as Indexer
   ```shell
   ./target/release/ord \
       --first-inscription-height=0 \
       --rpc-url=http://1234:1234@127.0.0.1:32552 \
       --data-dir=/mnt/ord-node/indexer-data \
       --index-transactions \
       --index-drc20 \
       --nr-parallel-requests=16 \
       --index-dunes \
       index
   ```

   - **Flags:**
     - `--index-transactions`: Enables storing transaction data (required for `--index-drc20`).
     - `--index-drc20`: Enables indexing DRC-20 tokens.
     - `--nr-parallel-requests`: Configures parallel requests for better performance (default: 16).

   Ensure that your Flopcoin node is synced and reachable at the RPC URL before starting the indexer or server.

---

## Running in Docker (Optional)

### Prerequisites
1. Ensure Flopcoin Core is fully synced.
2. Install Docker and Docker Compose:
   ```shell
   sudo apt update
   sudo apt install docker docker-compose
   ```

### Build and Run
1. Build the Docker image:
   ```shell
   docker build -t wonky-ord-flopcoin .
   ```

2. Start the indexer or server in a container:
   ```shell
   docker compose up -d
   ```

3. Stop the indexer or server with a timeout to prevent database corruption:
   ```shell
   docker compose stop -t 600
   docker compose down
   ```

---

## Notes

- Database size: The indexer requires ~400GB of storage when fully indexed with all features enabled.
- For more details, visit the [original README](READMEFROMAPEZORD.md) or refer to the [Flopcoin repository](https://github.com/Flopcoin/Flopcoin).
