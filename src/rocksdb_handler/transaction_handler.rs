///! Implementation of sqlite3 data handler
use crate::data_handler::{DataError, TransactionHandler, DataType};
use crate::transaction::Transaction;

use super::RocksDB;
use chrono::{Utc, DateTime};
use rocksdb::{IteratorMode, Direction};

impl TransactionHandler for RocksDB<'_> {
    fn get_transaction_by_id(&mut self, sort_prefix: &str, id: u128) -> Result<Transaction, DataError> {
        let key = self.build_key(
            DataType::Transaction,
            &sort_prefix,
            &id.to_string(),
        );

        match self.db.get(key) {
            Ok(Some(data)) =>
                match bincode::deserialize(&data) {
                    Ok(asset) => Ok(asset),
                    _ => Err(DataError::DataAccessFailure),
                },
            _ => Err(DataError::NotFound),
        }
    }

    fn get_latest_transaction(&mut self, sort_prefix: &str) -> Option<Transaction> {
        let quote_prefix = self.build_key(
            DataType::Transaction,
            &sort_prefix,
            "",
        );

        self.db.iterator(
            IteratorMode::From(quote_prefix.as_bytes(), Direction::Forward)
        )
            .next()
            .map(|item|
                match bincode::deserialize::<Transaction>(&item.1) {
                    Ok(quote) => Some(quote),
                    _ => None,
                }
            )
            .unwrap_or(None)
    }

    fn get_oldest_transaction(&mut self, sort_prefix: &str) -> Option<Transaction> {
        let quote_prefix = self.build_key(
            DataType::Transaction,
            &sort_prefix,
            &"\x7f",
        );

        self.db.iterator(
            IteratorMode::From(quote_prefix.as_bytes(), Direction::Forward)
        )
            .next()
            .map(|item|
                match bincode::deserialize::<Transaction>(&item.1) {
                    Ok(quote) => Some(quote),
                    _ => None,
                }
            )
            .unwrap_or(None)
    }

    fn insert_transaction(&mut self, sort_prefix: &str, transaction: &Transaction) -> Result<(), DataError> {
        self.update_transaction(sort_prefix, transaction)
    }

    fn update_transaction(&mut self, sort_prefix: &str, transaction: &Transaction) -> Result<(), DataError> {
        let key = self.build_key(
            DataType::Transaction,
            &sort_prefix,
            &transaction.id.to_string(),
        );

        match self.db.put(
            key,
            bincode::serialize(&transaction).unwrap(),
        ) {
            Ok(_) => Ok(()),
            Err(_) => DataError::InsertFailed,
        }
    }

    fn delete_transaction(&mut self, sort_prefix: &str, transaction: &Transaction) -> Result<(), DataError> {
        let key = self.build_key(
            DataType::Transaction,
            &sort_prefix,
            &transaction.id.to_string(),
        );

        match self.db.delete(key) {
            Ok(_) => Ok(()),
            Err(_) => DataError::DeleteFailed,
        }
    }

    fn transaction_cursor_forward(&mut self, sort_prefix: &str, time: DateTime<Utc>) -> Result<dyn Iterator<Item=Transaction>, DataError> {
        let quote_prefix = self.build_key(
            DataType::Transaction,
            &sort_prefix,
            &time.timestamp_nanos().to_string(),
        );

        Ok(
            self.db
                .iterator(
                    IteratorMode::From(quote_prefix.as_bytes(), Direction::Forward)
                )
                .map(|item| bincode::deserialize::<Transaction>(&item.1))
        )
    }

    fn transaction_cursor_reverse(&mut self, sort_prefix: &str, time: DateTime<Utc>) -> Result<dyn Iterator<Item=Transaction>, DataError> {
        let quote_prefix = self.build_key(
            DataType::Transaction,
            &sort_prefix,
            &time.timestamp_nanos().to_string(),
        );

        Ok(
            self.db
                .iterator(
                    IteratorMode::From(quote_prefix.as_bytes(), Direction::Reverse)
                )
                .map(|item| bincode::deserialize::<Transaction>(&item.1))
        )
    }
}
