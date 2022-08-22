use std::collections::HashMap;
use serde::Deserialize;

/// OptionChain Model
#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionChain {
    pub symbol: String,
    pub status: String,
//    pub underlying: String - ignore for now
    pub strategy: String,
    pub interval: f64,
    pub is_delayed: bool,
    pub is_index: bool,
    pub interest_rate: f64,
    #[serde(default)]
    pub underlying_price: f64,
    pub volatility: f64,
//    pub days_to_expiration: i64 - not sure what this is at top level
    pub number_of_contracts: i64,
    // format is HashMap< expiredate, HashMap< strike, [optionquote] > >
    #[serde(default)]
    pub put_exp_date_map: HashMap<String, HashMap<String, Vec<OptionQuote>>>,
    #[serde(default)]
    pub call_exp_date_map: HashMap<String, HashMap<String, Vec<OptionQuote>>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionQuote {
    pub put_call: String,
    pub symbol: String,
    pub description: String,
    pub exchange_name: String,
    pub bid: f64,
    pub ask: f64,
    pub last: f64,
    pub mark: f64,
    pub bid_size: i64,
    pub ask_size: i64,
    pub bid_ask_size: String,
    pub last_size: i64,
    pub high_price: f64,
    pub low_price: f64,
    pub open_price: f64,
    pub close_price: f64,
    pub total_volume: i64,
//    pub trade_date: Value,
    pub trade_time_in_long: i64,
    pub quote_time_in_long: i64,
    pub net_change: f64,
    pub volatility: f64,
    pub delta: f64,
    pub gamma: f64,
    pub theta: f64,
    pub vega: f64,
    pub rho: f64,
    pub open_interest: i64,
    pub time_value: f64,
    pub theoretical_option_value: f64,
    pub theoretical_volatility: f64,
//    pub option_deliverables_list: Value,
    pub strike_price: f64,
    pub expiration_date: i64,
    pub days_to_expiration: i64,
    pub expiration_type: String,
    pub last_trading_day: i64,
    pub multiplier: f64,
    pub settlement_type: String,
    pub deliverable_note: String,
//    pub is_index_option: Value,
    pub percent_change: f64,
    pub mark_change: f64,
    pub mark_percent_change: f64,
    pub intrinsic_value: f64,
    pub non_standard: bool,
    pub penny_pilot: bool,
    pub in_the_money: bool,
    pub mini: bool,
}
