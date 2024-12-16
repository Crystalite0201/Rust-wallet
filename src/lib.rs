use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::api;
use ic_cdk_macros::{init, query, update};
use std::cell::RefCell;

// A simple data structure for the wallet
#[derive(CandidType, Deserialize, Clone)]
struct Transaction {
    to: String,
    amount: u64,
    timestamp: u64,
}

#[derive(CandidType, Deserialize)]
struct Wallet {
    balance: u64,
    transactions: Vec<Transaction>,
}

// Thread-local storage for the wallet
thread_local! {
    static WALLET: RefCell<Wallet> = RefCell::new(Wallet {
        balance: 0,
        transactions: Vec::new(),
    });
}

// Initialize the wallet
#[init]
fn init_wallet() {
    ic_cdk::print("Wallet initialized.");
}

// Fetch the wallet balance
#[query]
fn get_balance() -> u64 {
    WALLET.with(|wallet| wallet.borrow().balance)
}

// Send tokens to another principal
#[update]
async fn send_tokens(to: String, amount: u64) -> Result<String, String> {
    // Simulate inter-canister call to transfer tokens
    let caller = api::caller().to_string();

    // Check for sufficient balance
    let mut new_balance = 0;
    WALLET.with(|wallet| {
        let mut w = wallet.borrow_mut();
        if w.balance < amount {
            return Err("Insufficient balance".to_string());
        }
        w.balance -= amount;
        new_balance = w.balance;

        // Record the transaction
        w.transactions.push(Transaction {
            to: to.clone(),
            amount,
            timestamp: api::time(),
        });
    });

    // Simulate the transfer
    ic_cdk::api::call::call::<_, ()>(
        to.parse().unwrap(),
        "transfer",
        (caller, to.clone(), amount),
    )
    .await
    .map(|_| {
        Ok()
    })
    .err()

}

// Query for tokens
    // Log successful transfer
    Ok(format!("Transfer successful. New balance: {}", new_balance))
}

// Update balance when tokens are received
#[update]
fn receive_tokens(amount: u64) {
    WALLET.with(|wallet| {
        let mut w = wallet.borrow_mut();
        w.balance += amount;

        // Log the receipt
        w.transactions.push(Transaction {
            to: api::caller().to_string(),
            amount,
            timestamp: api::time(),
        });
    });
    ic_cdk::print("Tokens received successfully.");
}

// Fetch the transaction history
#[query]
fn get_transactions() -> Vec<Transaction> {
    WALLET.with(|wallet| wallet.borrow().transactions.clone())
}
