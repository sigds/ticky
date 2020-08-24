use super::DataError;
use crate::asset::Asset;
use crate::currency::Currency;

/// Handler for globally available data of transactions and related data
pub trait AssetHandler {
    fn get_asset_by_name(&mut self, id: usize) -> Result<Asset, DataError>;

    fn insert_asset(&mut self, asset: &Asset) -> Result<usize, DataError>;
    fn update_asset(&mut self, asset: &Asset) -> Result<(), DataError>;
    fn delete_asset(&mut self, id: usize) -> Result<(), DataError>;

    fn insert_asset_if_needed(
        &mut self,
        asset: &Asset,
    ) -> Result<usize, DataError> {
        match self.get_asset_id(asset) {
            Some(id) => Ok(id),
            None => self.insert_asset(asset),
        }
    }
}
