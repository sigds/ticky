///! Implementation of a data handler trait to deal with global data
use std::fmt;

pub mod asset_handler;
pub mod quote_handler;
pub mod transaction_handler;

pub use asset_handler::AssetHandler;
pub use quote_handler::QuoteHandler;
pub use transaction_handler::TransactionHandler;

#[derive(Debug, Copy)]
pub enum DataType {
    Asset,
    Quote,
    Ticker,
    Transaction,
}

#[derive(Debug)]
pub enum DataError {
    DataAccessFailure,
    NotFound,
    UpdateFailed,
    DeleteFailed,
    InsertFailed,
    InvalidTransaction,
}

pub trait DataItem {
    // get id or return error if id hasn't been set yet
    fn get_id(&self) -> Result<usize, DataError>;
    // set id or return error if id has already been set
    fn set_id(&mut self, id: usize) -> Result<(), DataError>;
}
