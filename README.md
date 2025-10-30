# 🪙 Blocksmith Transaction Client

This is a test client for the Blocksmith blockchain node. It automatically generates wallets and periodically sends
random signed transactions to one or more running nodes.

⚠️ Note: this tool is for testing and benchmarking only — not intended for production or real cryptocurrency use.

------------------------------------------------------------------------------------------------------------------------

## ⚙️ What It Does

- Generates 100 random wallets (using `secp256k1`).
- Randomly selects sender and receiver wallets.
- Signs each transaction with the sender's private key.
- Sends transactions to random nodes listed in the `NODES` environment variable.
- Repeats the process every 5 seconds.

------------------------------------------------------------------------------------------------------------------------

## 🛠️ Usage

1. Set up environment

    Create a `.env` file or export `NODES` environmental variable manually:

    ```shell
    export NODES=http://127.0.0.1:60606,http://127.0.0.1:60607,http://127.0.0.1:60608
    ```

2. Run the client

    ```shell
    cargo run
    ```

The client will start generating and sending signed transactions to your running nodes.

------------------------------------------------------------------------------------------------------------------------

# 💬 Example Use Case

Run this client alongside one or more Blocksmith nodes to:

- test transaction propagation;
- simulate network load;
- validate mempool and block inclusion logic;
- demonstrate transaction signing and verification.
