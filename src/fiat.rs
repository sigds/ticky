use crate::data_handler::DataError;
use std::ops::Neg;
use std::fmt::{Display, Formatter};
use std::fmt;
use std::error::Error;
use chrono::NaiveDate;

/// Currency of an amunt of money
#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq)]
pub enum Currency {
    AED,
    AFN,
    ALL,
    AMD,
    ANG,
    AOA,
    ARS,
    AUD,
    AWG,
    AZN,
    BAM,
    BBD,
    BDT,
    BGN,
    BHD,
    BIF,
    BMD,
    BND,
    BOB,
    BOV,
    BRL,
    BSD,
    BTN,
    BWP,
    BYR,
    BZD,
    CAD,
    CDF,
    CHE,
    CHF,
    CHW,
    CLF,
    CLP,
    CNY,
    COP,
    COU,
    CRC,
    CUC,
    CUP,
    CVE,
    CZK,
    DJF,
    DKK,
    DOP,
    DZD,
    EGP,
    ERN,
    ETB,
    EUR,
    FJD,
    FKP,
    GBP,
    GEL,
    GHS,
    GIP,
    GMD,
    GNF,
    GTQ,
    GYD,
    HKD,
    HNL,
    HRK,
    HTG,
    HUF,
    IDR,
    ILS,
    INR,
    IQD,
    IRR,
    ISK,
    JMD,
    JOD,
    JPY,
    KES,
    KGS,
    KHR,
    KMF,
    KPW,
    KRW,
    KWD,
    KYD,
    KZT,
    LAK,
    LBP,
    LKR,
    LRD,
    LSL,
    LTL,
    LVL,
    LYD,
    MAD,
    MDL,
    MGA,
    MKD,
    MMK,
    MNT,
    MOP,
    MRO,
    MUR,
    MVR,
    MWK,
    MXN,
    MXV,
    MYR,
    MZN,
    NAD,
    NGN,
    NIO,
    NOK,
    NPR,
    NZD,
    OMR,
    PAB,
    PEN,
    PGK,
    PHP,
    PKR,
    PLN,
    PYG,
    QAR,
    RON,
    RSD,
    RUB,
    RWF,
    SAR,
    SBD,
    SCR,
    SDG,
    SEK,
    SGD,
    SHP,
    SLL,
    SOS,
    SRD,
    SSP,
    STD,
    SYP,
    SZL,
    THB,
    TJS,
    TMT,
    TND,
    TOP,
    TRY,
    TTD,
    TWD,
    TZS,
    UAH,
    UGX,
    USD,
    USN,
    USS,
    UYI,
    UYU,
    UZS,
    VEF,
    VND,
    VUV,
    WST,
    XAF,
    XAG,
    XAU,
    XBA,
    XBB,
    XBC,
    XBD,
    XCD,
    XDR,
    XFU,
    XOF,
    XPD,
    XPF,
    XPT,
    XTS,
    XXX,
    YER,
    ZAR,
    ZMW,
}

/// Container for an amount of money in some currency
#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq)]
pub struct CashAmount {
    pub amount: f64,
    pub currency: Currency,
}

pub fn round2digits(x: f64, digits: i32) -> f64 {
    (x * 10.0_f64.powi(digits)).round() / 10.0_f64.powi(digits)
}

impl CashAmount {
    pub fn add(
        &mut self,
        cash_amount: CashAmount,
    ) -> Result<&mut Self, dyn Error> {
        if self.currency == cash_amount.currency {
            self.amount += cash_amount.amount;
            Ok(self)
        } else {
            Err(self)
        }
    }

    pub fn sub(
        &mut self,
        cash_amount: CashAmount,
    ) -> Result<&mut Self, dyn Error> {
        if self.currency == cash_amount.currency {
            self.amount -= cash_amount.amount;

            Ok(self)
        } else {
            Err(self)
        }
    }
}

impl Display for CashAmount {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:16.4} {:?}", self.amount, self.currency)
    }
}

impl Neg for CashAmount {
    type Output = CashAmount;

    fn neg(self) -> Self::Output {
        CashAmount {
            amount: -self.amount,
            currency: self.currency,
        }
    }
}

/// Container for a single cash flow
#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub struct CashFlow {
    pub amount: CashAmount,
    pub date: NaiveDate,
}

impl CashFlow {
    /// Construct new cash flow
    pub fn new(amount: f64, currency: Currency, date: NaiveDate) -> CashFlow {
        CashFlow {
            amount: CashAmount { amount, currency },
            date,
        }
    }
    /// Check, whether cash flows could be aggregated
    pub fn aggregatable(&self, cf: &CashFlow) -> bool {
        if self.amount.currency != cf.amount.currency {
            false
        } else if self.date != cf.date {
            false
        } else {
            true
        }
    }

    /// Compare to cash flows for equality within a given absolute tolerance
    pub fn fuzzy_cash_flows_cmp_eq(&self, cf: &CashFlow, tol: f64) -> bool {
        if !self.aggregatable(cf) {
            false
        } else if self.amount.amount.is_nan()
            || cf.amount.amount.is_nan()
            || (self.amount.amount - cf.amount.amount).abs() > tol
        {
            false
        } else {
            true
        }
    }
}

impl Display for CashFlow {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.date, self.amount)
    }
}

impl Neg for CashFlow {
    type Output = CashFlow;

    fn neg(self) -> Self::Output {
        CashFlow {
            amount: -self.amount,
            date: self.date,
        }
    }
}
