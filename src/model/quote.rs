use std::collections::HashMap;
use serde_json::Value;
use serde::{Deserialize, Serialize};
///
/// Quote
///
/// Deserialized through a HashMap<String, Quote>
/// 
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"), untagged)]
pub enum Quote {
    Equity(QEquity),
    Index(QIndex),
    Option(QOption),
    Fund(QFund),
    General(QGeneral),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct QEquity {
    pub asset_type: String,    
    pub symbol: String,
    pub description: String,
    pub bid_price: f64,
    pub bid_size: f64,
    pub bid_id: String,
    pub ask_price: f64,
    pub ask_size: f64,
    pub ask_id: String,
    pub last_price: f64,
    pub last_size: f64,
    pub last_id: String,
    pub open_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub close_price: f64,
    pub net_change: f64,
    pub total_volume: i64,
    pub quote_time_in_long: i64,
    pub trade_time_in_long: i64,
    pub mark: f64,
    pub cusip: String,
    pub exchange: String,
    pub exchange_name: String,
    pub marginable: bool,
    pub shortable: bool,
    pub volatility: f64,
    pub digits: i64,
    #[serde(rename = "52WkHigh")]
    pub n52wk_high: f64,
    #[serde(rename = "52WkLow")]
    pub n52wk_low: f64,
    pub pe_ratio: f64,
    pub div_amount: f64,
    pub div_yield: f64,
    pub div_date: String,
    pub security_status: String,
    pub regular_market_last_price: f64,
    pub regular_market_last_size: f64,
    pub regular_market_net_change: f64,
    pub regular_market_trade_time_in_long: f64,
    pub net_percent_change_in_double: f64,
    pub mark_change_in_double: f64,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct QIndex {
    pub asset_type: String,    
    pub symbol: String,
    pub description: String,
    pub last_price: f64,
    pub open_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub close_price: f64,
    pub net_change: f64,
    pub total_volume: i64,
    pub trade_time_in_long: i64,
    pub exchange: String,
    pub exchange_name: String,
    pub digits: i64,
    #[serde(rename = "52WkHigh")]
    pub n52wk_high: f64,
    #[serde(rename = "52WkLow")]
    pub n52wk_low: f64,
    pub security_status: String,
    pub net_percent_change_in_double: f64,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}   

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct QOption {
    pub asset_type: String,    
    pub symbol: String,
    pub description: String,
    pub bid_price: f64,
    pub bid_size: f64,
    pub ask_price: f64,
    pub ask_size: f64,
    pub last_price: f64,
    pub last_size: f64,
    pub open_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub close_price: f64,
    pub net_change: f64,
    pub total_volume: i64,
    pub quote_time_in_long: i64,
    pub trade_time_in_long: i64,
    pub mark: f64,
    pub cusip: String,
    pub open_interest: f64,
    pub volatility: f64,
    pub money_intrinsic_value: f64,
    pub multiplier: f64,
    pub strike_price: f64,
    pub contract_type: String,
    pub underlying: String,
    pub time_value: f64,
    pub deliverables: String,
    pub delta: f64,
    pub gamma: f64,
    pub theta: f64,
    pub vega: f64,
    pub rho: f64,
    pub security_status: String,
    pub theoretical_option_value: f64,
    pub underlying_price: f64,
    pub uv_expiration_type: String,
    pub exchange: String,
    pub exchange_name: String,
    pub settlement_type: String,
    pub net_percent_change_in_double: f64,
    pub mark_change_in_double: f64,
    pub last_trading_day: i64,
    pub expiration_day: i64,
    pub expiration_month: i64,
    pub expiration_year: i64,
    pub days_to_expiration: i64,
    pub implied_yield: f64,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct QFund {
    pub asset_type: String,    
    pub symbol: String,
    pub description: String,
    pub close_price: f64,
    pub net_change: f64,
    pub total_volume: i64,
    pub trade_time_in_long: i64,
    pub cusip: String,
    pub exchange: String,
    pub exchange_name: String,
    pub digits: i64,
    #[serde(rename = "52WkHigh")]
    pub n52wk_high: f64,
    #[serde(rename = "52WkLow")]
    pub n52wk_low: f64,
    #[serde(rename = "nAV")]
    pub nav: f64,
    pub pe_ratio: f64,
    pub div_amount: f64,
    pub div_yield: f64,
    pub div_date: String,
    pub security_status: String,
    pub net_percent_change_in_double: f64,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub struct QGeneral {
    pub asset_type: String,    
    pub symbol: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}


impl Quote {
    pub fn symbol(&self) -> &str {
        match self {
            Quote::Equity(quote) => &quote.symbol,
            Quote::Option(quote) => &quote.symbol,
            Quote::Fund(quote) => &quote.symbol,
            Quote::Index(quote) => &quote.symbol,
            Quote::General(quote) => &quote.symbol,
        }
    }
    pub fn mark_price(&self) -> f64 {
        match self {
            Quote::Equity(quote) => quote.mark,
            Quote::Option(quote) => quote.mark,
            Quote::Fund(quote) => quote.nav,
            Quote::Index(quote) => quote.last_price,
            _ => 0.0,
        }
    }
    pub fn close_price(&self) -> f64 {
        match self {
            Quote::Equity(quote) => quote.close_price,
            Quote::Option(quote) => quote.close_price,
            Quote::Fund(quote) => quote.close_price,
            Quote::Index(quote) => quote.close_price,
            _ => 0.0,
        }
    }
}

// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=cf9cda3d15e7bb0cc905047e11d42c78