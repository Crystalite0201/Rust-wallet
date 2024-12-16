# Rust-wallet

## Overview
This is a Rust-based token wallet for the Internet Computer Protocol (ICP) blockchain. The wallet supports sending and receiving IRCRC2 tokens, viewing balances, and transaction history.

## Features
- Send Tokens
- Receive Tokens
- Display Wallet Balance
- Transaction History

## Setup
1. Install Rust and the DFINITY SDK.
2. Clone the repository:
https://github.com/Crystalit0201/Rust-wallet.git
3. Build the wallet:
cargo build --target wasm32-unknown-unknown --release
4. Deploy to a local ICP network:
dfx start --background dfx deploy wallet

## Testing
Run the unit tests:
cargo test

## Usage
### Check Balance
dfx canister call wallet get_balance

### Receive Tokens
dfx canister call wallet receive_tokens '(100)'

### Send Tokens
dfx canister call wallet send_tokens '("recipient-id", 50)'

### Transaction History
dfx canister call wallet get_transactions
