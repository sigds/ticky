///! Implementation of a container for basic asset data


use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


use crate::fiat::Currency;


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
