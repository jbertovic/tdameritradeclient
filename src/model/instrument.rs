use serde::Deserialize;
///
/// Instrument
///
/// Deserialized through a HashMap<String, Instrument>
/// String is Symbol
///
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase", untagged)]
pub enum InstrumentSearch {
    Fundamental(FundamentalDetails),
    Instrument(InstrumentDetails),
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FundamentalDetails {
    pub fundamental: Fundamental,
    pub cusip: String,
    pub symbol: String,
    pub description: String,
    pub exchange: String,
    pub asset_type: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Fundamental {
    pub symbol: String,
    pub high52: f64,
    pub low52: f64,
    pub dividend_amount: f64,
    pub dividend_yield: f64,
    pub dividend_date: String,
    pub pe_ratio: f64,
    pub peg_ratio: f64,
    pub pb_ratio: f64,
    pub pr_ratio: f64,
    pub pcf_ratio: f64,
    #[serde(rename = "grossMarginTTM")]
    pub gross_margin_ttm: f64,
    #[serde(rename = "grossMarginMRQ")]
    pub gross_margin_mrq: f64,
    #[serde(rename = "netProfitMarginTTM")]
    pub net_profit_margin_ttm: f64,
    #[serde(rename = "netProfitMarginMRQ")]
    pub net_profit_margin_mrq: f64,
    #[serde(rename = "operatingMarginTTM")]
    pub operating_margin_ttm: f64,
    #[serde(rename = "operatingMarginMRQ")]
    pub operating_margin_mrq: f64,
    pub return_on_equity: f64,
    pub return_on_assets: f64,
    pub return_on_investment: f64,
    pub quick_ratio: f64,
    pub current_ratio: f64,
    pub interest_coverage: f64,
    pub total_debt_to_capital: f64,
    pub lt_debt_to_equity: f64,
    pub total_debt_to_equity: f64,
    #[serde(rename = "epsTTM")]
    pub eps_ttm: f64,
    #[serde(rename = "epsChangePercentTTM")]
    pub eps_change_percent_ttm: f64,
    pub eps_change_year: f64,
    pub eps_change: f64,
    pub rev_change_year: f64,
    #[serde(rename = "revChangeTTM")]
    pub rev_change_ttm: f64,
    pub rev_change_in: f64,
    pub shares_outstanding: f64,
    pub market_cap_float: f64,
    pub market_cap: f64,
    pub book_value_per_share: f64,
    pub short_int_to_float: f64,
    pub short_int_day_to_cover: f64,
    #[serde(rename = "divGrowthRate3Year")]
    pub div_growth_rate3year: f64,
    pub dividend_pay_amount: f64,
    pub dividend_pay_date: String,
    pub beta: f64,
    #[serde(rename = "vol1DayAvg")]
    pub vol1day_avg: f64,
    #[serde(rename = "vol10DayAvg")]
    pub vol10day_avg: f64,
    #[serde(rename = "vol3MonthAvg")]
    pub vol3month_avg: f64,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentDetails {
    pub cusip: String,
    pub symbol: String,
    pub description: String,
    pub exchange: String,
    pub asset_type: String,
}
