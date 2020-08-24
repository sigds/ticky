///! Implemenation of sqlite3 data handler
use super::RocksDB;

use crate::asset::Asset;
use crate::data_handler::{AssetHandler, DataError, DataType};
use crate::currency::Currency;
use std::str::FromStr;

impl AssetHandler for RocksDB<'_> {
    fn get_asset_by_name(&mut self, name: &str) -> Result<Asset, DataError> {
        let key = self.build_key(
            DataType::Asset,
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

    fn insert_asset(&mut self, asset: &Asset) -> Result<(), DataError> {
        self.update_asset(asset)
    }

    fn update_asset(&mut self, asset: &Asset) -> Result<(), DataError> {
        let key = self.build_key(
            DataType::Asset,
            &asset.name,
            "",
        );

        match self.db.put(
            key,
            bincode::serialize(&asset).unwrap(),
        ) {
            Ok(_) => Ok(()),
            Err(_) => DataError::InsertFailed,
        }
    }

    fn delete_asset(&mut self, asset: &Asset) -> Result<(), DataError> {
        let key = self.build_key(
            DataType::Asset,
            &asset.name,
            "",
        );

        match self.db.delete(key) {
            Ok(_) => Ok(()),
            Err(_) => DataError::DeleteFailed,
        }
    }
}
