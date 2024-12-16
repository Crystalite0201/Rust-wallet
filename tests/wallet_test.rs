#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_balance() {
        init_wallet();
        assert_eq!(get_balance(), 0);
    }

    #[test]
    fn test_receive_tokens() {
        receive_tokens(100);
        assert_eq!(get_balance(), 100);
    }

    #[test]
    fn test_send_tokens() {
        receive_tokens(200);
        let result = send_tokens("recipient-id".to_string(), 50);
        assert!(result.is_ok());
        assert_eq!(get_balance(), 150);
    }

    #[test]
    fn test_transaction_history() {
        receive_tokens(300);
        send_tokens("recipient-id".to_string(), 100).unwrap();
        let history = get_transactions();
        assert_eq!(history.len(), 1);
    }
}
