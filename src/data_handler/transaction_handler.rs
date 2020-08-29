use super::AssetHandler;
use super::DataError;
use crate::transaction::Transaction;
use chrono::{DateTime, Utc};

/// Handler for globally available data of transactions and related data
pub trait TransactionHandler: AssetHandler {
    fn get_transaction_by_id(&mut self, sort_prefix: &str, id: u128) -> Result<Transaction, DataError>;

    fn get_latest_transaction(&mut self, sort_prefix: &str) -> Option<Transaction>;
    fn get_oldest_transaction(&mut self, sort_prefix: &str) -> Option<Transaction>;

    fn insert_transaction(&mut self, sort_prefix: &str, transaction: &Transaction) -> Result<(), DataError>;
    fn update_transaction(&mut self, sort_prefix: &str, transaction: &Transaction) -> Result<(), DataError>;
    fn delete_transaction(&mut self, sort_prefix: &str, transaction: &Transaction) -> Result<(), DataError>;

    fn transaction_cursor_forward(&mut self, sort_prefix: &str, time: DateTime<Utc>) -> Box<dyn Iterator<Item=Transaction> + '_>;
    fn transaction_cursor_reverse(&mut self, sort_prefix: &str, time: DateTime<Utc>) -> Box<dyn Iterator<Item=Transaction> + '_>;
}
