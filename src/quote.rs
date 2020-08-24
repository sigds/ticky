///! Implementation of a container for basic asset data
use crate::data_handler::{DataError, DataItem};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use crate::fiat::Currency;
use crate::asset::Asset;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticker {
    pub name: String,
    pub asset: String,
    pub currency: Currency,
    pub priority: i32,
    pub factor: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    // sequence id
    pub id: Option<i64>,
    pub ticker: String,
    pub price: f64,
    pub time: DateTime<Utc>,
    pub volume: Option<f64>,
}
