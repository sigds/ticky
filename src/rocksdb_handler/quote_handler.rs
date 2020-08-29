///! Implementation for quote handler with Sqlite3 database as backend
use super::RocksDB;

use crate::data_handler::{DataError, QuoteHandler, DataType};

use crate::quote::{Quote, Ticker};
use chrono::{DateTime, Utc};

use rocksdb::{IteratorMode, Direction};


/// Sqlite implementation of quote handler
impl QuoteHandler for RocksDB {
    fn get_ticker_by_name(&mut self, name: &str) -> Result<Ticker, DataError> {
        let key = self.build_key(
            &DataType::Ticker,
            &name,
            "",
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

    fn get_latest_quote(&mut self, ticker_name: &str) -> Option<Quote> {
        let quote_prefix = self.build_key(
            &DataType::Quote,
            &ticker_name,
            "",
        );

        self.db
            .iterator(
                IteratorMode::From(&quote_prefix, Direction::Forward)
            )
            .next()
            .map(|item|
                match bincode::deserialize::<Quote>(&item.1) {
                    Ok(quote) => Some(quote),
                    _ => None,
                }
            )
            .unwrap_or(None)
    }

    fn get_oldest_quote(&mut self, ticker_name: &str) -> Option<Quote> {
        let quote_prefix = self.build_key(
            &DataType::Quote,
            &ticker_name,
            "\x7f",
        );

        self.db
            .iterator(
                IteratorMode::From(&quote_prefix, Direction::Reverse)
            )
            .next()
            .map(|item|
                match bincode::deserialize::<Quote>(&item.1) {
                    Ok(quote) => Some(quote),
                    _ => None,
                }
            )
            .unwrap_or(None)
    }

    fn insert_ticker(&mut self, ticker: &Ticker) -> Result<(), DataError> {
        self.update_ticker(ticker)
    }

    fn update_ticker(&mut self, ticker: &Ticker) -> Result<(), DataError> {
        let key = self.build_key(
            &DataType::Ticker,
            &ticker.name,
            "",
        );

        self.db
            .put(
                key,
                bincode::serialize(&ticker).unwrap(),
            )
            .map_err(|_| DataError::InsertFailed)
    }

    fn delete_ticker(&mut self, ticker: &Ticker) -> Result<(), DataError> {
        let key = self.build_key(
            &DataType::Ticker,
            &ticker.name,
            "",
        );

        self.db
            .delete(key)
            .map_err(|_| DataError::DeleteFailed)
    }

    fn insert_quote(&mut self, quote: &Quote) -> Result<(), DataError> {
        self.update_quote(quote)
    }

    fn update_quote(&mut self, quote: &Quote) -> Result<(), DataError> {
        let ticker_key = self.build_key(
            &DataType::Quote,
            &quote.ticker,
            &quote.time.timestamp_nanos().to_string(),
        );

        let key = self.build_subkey(
            &ticker_key,
            quote.id.unwrap_or(0).to_string().as_bytes(),
        );

        self.db
            .put(
                key,
                bincode::serialize(&quote).unwrap(),
            )
            .map_err(|_| DataError::InsertFailed)
    }

    fn delete_quote(&mut self, quote: &Quote) -> Result<(), DataError> {
        let ticker_key = self.build_key(
            &DataType::Quote,
            &quote.ticker,
            &quote.time.timestamp_nanos().to_string(),
        );

        let key = self.build_subkey(
            &ticker_key,
            quote.id.unwrap_or(0).to_string().as_bytes(),
        );


        self.db
            .delete(key)
            .map_err(|_| DataError::DeleteFailed)
    }

    fn quote_cursor_forward(&mut self, ticker: &Ticker, time: DateTime<Utc>) -> Box<dyn Iterator<Item=Quote> + '_> {
        let quote_prefix = self.build_key(
            &DataType::Quote,
            &ticker.name,
            &time.timestamp_nanos().to_string(),
        );

        let iter =
            self.db
                .iterator(
                    IteratorMode::From(&quote_prefix, Direction::Forward)
                )
                .filter_map(|item|
                    bincode::deserialize::<Quote>(&item.1)
                        .ok()
                );

        Box::new(
            iter
        )
    }

    fn quote_cursor_reverse(&mut self, ticker: &Ticker, time: DateTime<Utc>) -> Box<dyn Iterator<Item=Quote> + '_> {
        let quote_prefix = self.build_key(
            &DataType::Quote,
            &ticker.name,
            &time.timestamp_nanos().to_string(),
        );

        let iter =
            self.db
                .iterator(
                    IteratorMode::From(&quote_prefix, Direction::Reverse)
                )
                .filter_map(|item|
                    match bincode::deserialize::<Quote>(&item.1) {
                        Ok(res) => Some(res.to_owned()),
                        Err(_) => None
                    }
                );

        Box::new(
            iter
        )
    }
}
