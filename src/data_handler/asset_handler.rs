use super::DataError;
use crate::asset::Asset;

/// Handler for globally available data of transactions and related data
pub trait AssetHandler {
    fn get_asset_by_name(&mut self, name: &str) -> Result<Asset, DataError>;

    fn insert_asset(&mut self, asset: &Asset) -> Result<(), DataError>;
    fn update_asset(&mut self, asset: &Asset) -> Result<(), DataError>;
    fn delete_asset(&mut self, asset: &Asset) -> Result<(), DataError>;
}
