
use serde::{Deserialize, Serialize};
use crate::fiat::CashFlow;
use std::time::{SystemTime, UNIX_EPOCH};

/// Type of transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionType {
    Cash,
    Asset { asset_name: String, position: f64 },
    Dividend { asset_name: String },
    Interest { asset_name: String },
    Tax { transaction_ref: Option<u128> },
    Fee { transaction_ref: Option<u128> },
}

/// Basic transaction data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: u128,
    pub transaction_type: TransactionType,
    pub cash_flow: CashFlow,
    pub note: Option<String>,
}

impl Transaction {
    pub fn new(
        transaction_type: TransactionType,
        cash_flow: CashFlow,
        note: Option<String>,
    ) -> Transaction {
        let start = SystemTime::now();
        let time_since_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards, cannot safely create transaction id.");

        Transaction {
            id: time_since_epoch.as_nanos(),
            transaction_type,
            cash_flow,
            note,
        }
    }

    /// Assign or change transaction's asset_id, if possible
    /// This is often required for transactions on new assets
    pub fn set_asset_name(&mut self, asset_name: String) {
        let new_type = match self.transaction_type {
            TransactionType::Asset {
                asset_name: _,
                position,
            } => TransactionType::Asset { asset_name, position },
            TransactionType::Dividend { asset_name: _ } => TransactionType::Dividend { asset_name },
            TransactionType::Interest { asset_name: _ } => TransactionType::Interest { asset_name },
            _ => { return },
        };

        self.transaction_type = new_type;
    }

    /// Assign new transaction reference, if applicable
    pub fn set_transaction_ref(&mut self, trans_ref: u128) {
        let new_type = match self.transaction_type {
            TransactionType::Tax { transaction_ref: _ } => TransactionType::Tax {
                transaction_ref: Some(trans_ref),
            },
            TransactionType::Fee { transaction_ref: _ } => TransactionType::Fee {
                transaction_ref: Some(trans_ref),
            },
            _ => { return },
        };

        self.transaction_type = new_type;
    }
}
