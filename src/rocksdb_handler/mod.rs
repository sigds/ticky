///! Implemenation of rocksdb data handler
use rocksdb::DB;
use std::path::Path;
use crate::data_handler::DataType;

mod asset_handler;
mod quote_handler;
mod transaction_handler;

/// Struct to handle connections to sqlite3 databases
pub struct RocksDB {
    /// conn is made public to allow extending this struct outside of the library
    pub db: DB,
}

impl RocksDB {
    pub fn new<P: AsRef<Path>>(&self, path: P) -> Result<RocksDB, rocksdb::Error> {
        let db = rocksdb::DB::open_default(path);

        if db.is_err() {
            return Err(db.unwrap_err());
        }

        Ok(
            RocksDB {
                db: db.unwrap()
            },
        )
    }

    fn build_key(
        &self,
        data_type: &DataType,
        secondary_id: &str,
        tertiary_id: &str,
    ) -> Vec<u8> {
        format!(
            "{}:{}:{}",
            *data_type as u8,
            secondary_id,
            tertiary_id,
        ).as_bytes().to_vec()
    }

    fn build_subkey(
        &self,
        primary_key: &[u8],
        secondary_key: &[u8],
    ) -> Vec<u8> {
        (&[primary_key, secondary_key].concat()).to_vec()
    }
}
