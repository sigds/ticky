use crate::data_handler::{DataError, DataItem};
///! Implementation of a container for basic asset data
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetCategory {
    id: usize,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub name: String,
    pub wkn: Option<String>,
    pub isin: Option<String>,
    pub note: Option<String>,
}

impl Asset {
    pub fn new(
        name: &str,
        wkn: Option<String>,
        isin: Option<String>,
        note: Option<String>,
    ) -> Asset {
        Asset {
            name: name.to_string(),
            wkn,
            isin,
            note,
        }
    }
}
