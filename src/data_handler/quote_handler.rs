use super::AssetHandler;
///! Data handler trait for market quotes
use super::DataError;
use crate::currency::Currency;
use crate::quote::{MarketDataSource, Quote, Ticker};
use chrono::{DateTime, Utc};

/// Handler for globally available market quotes data
pub trait QuoteHandler: AssetHandler {
    fn get_ticker_by_name(&mut self, name: &str) -> Result<Ticker, DataError>;
    fn get_latest_quote(&mut self, ticker_name: &str) -> Option<Quote>;
    fn get_oldest_quote(&mut self, ticker_name: &str) -> Option<Quote>;

    fn insert_ticker(&mut self, ticker: &Ticker) -> Result<(), DataError>;
    fn update_ticker(&mut self, ticker: &Ticker) -> Result<(), DataError>;
    fn delete_ticker(&mut self, ticker: &Ticker) -> Result<(), DataError>;

    fn insert_quote(&mut self, quote: &Quote) -> Result<usize, DataError>;

    fn update_quote(&mut self, quote: &Quote) -> Result<(), DataError>;
    fn delete_quote(&mut self, quote: &Quote) -> Result<(), DataError>;

    fn quote_cursor_forward(&mut self, ticker: &Ticker, time: DateTime<Utc>) -> Result<dyn Iterator<Item=Quote>, DataError>;
    fn quote_cursor_reverse(&mut self, ticker: &Ticker, time: DateTime<Utc>) -> Result<dyn Iterator<Item=Quote>, DataError>;
}
