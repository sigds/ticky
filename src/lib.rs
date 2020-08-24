//! # ticky
//!
//! Purpose of this library is to provide over time a comprehensive toolbox
//! for quantitative analysis of financial assets in rust.
//!

// macro exports
#[macro_use]
pub mod macros;

// module exports
pub mod asset;
pub mod fiat;
pub mod data_handler;
pub mod date_time_helper;
pub mod helpers;
pub mod quote;
pub mod rocksdb_handler;
pub mod transaction;

pub use currency::Currency;
