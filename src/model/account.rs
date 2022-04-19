use serde::{Deserialize, Serialize};
///
/// Holds account information that contains account information, balances, positions and orders
/// as retrieved from get Accounts
///
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountRoot {
    pub securities_account: SecuritiesAccount,
}

// TODO: Add the opportunity for either cash_account or margin_account. Currently only uses margin_account
// Need to test cash account to see if it works

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecuritiesAccount {
    #[serde(rename = "type")]
    pub type_field: String,
    pub account_id: String,
    pub round_trips: i64,
    pub is_day_trader: bool,
    pub is_closing_only_restricted: bool,
    pub positions: Option<Vec<Position>>,
    pub order_strategies: Option<Vec<OrderStrategy>>,
    pub initial_balances: InitialBalances,
    pub current_balances: CurrentBalances,
    pub projected_balances: ProjectedBalances,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub short_quantity: f64,
    pub average_price: f64,
    pub current_day_profit_loss: f64,
    pub current_day_profit_loss_percentage: f64,
    pub long_quantity: f64,
    pub settled_long_quantity: f64,
    pub settled_short_quantity: f64,
    pub instrument: Instrument,
    pub market_value: f64,
    pub maintenance_requirement: f64,
    pub current_day_cost: f64,
    pub previous_session_long_quantity: Option<f64>,
    pub previous_session_short_quantity: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instrument {
    pub asset_type: String,
    pub cusip: String,
    pub symbol: String,
    pub description: Option<String>,
    pub put_call: Option<String>,
    pub underlying_symbol: Option<String>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitialBalances {
    pub accrued_interest: f64,
    #[serde(default)]
    pub available_funds_non_marginable_trade: f64,
    pub bond_value: f64,
    pub buying_power: f64,
    pub cash_balance: f64,
    pub cash_available_for_trading: f64,
    pub cash_receipts: f64,
    pub day_trading_buying_power: f64,
    pub day_trading_buying_power_call: f64,
    pub day_trading_equity_call: f64,
    #[serde(default)]
    pub equity: f64,
    #[serde(default)]
    pub equity_percentage: f64,
    pub liquidation_value: f64,
    #[serde(default)]
    pub long_margin_value: f64,
    pub long_option_market_value: f64,
    pub long_stock_value: f64,
    #[serde(default)]
    pub maintenance_call: f64,
    #[serde(default)]
    pub maintenance_requirement: f64,
    #[serde(default)]
    pub margin: f64,
    #[serde(default)]
    pub margin_equity: f64,
    pub money_market_fund: f64,
    pub mutual_fund_value: f64,
    #[serde(rename = "regTCall")]
    pub reg_tcall: f64,
    #[serde(default)]
    pub short_margin_value: f64,
    pub short_option_market_value: f64,
    pub short_stock_value: f64,
    pub total_cash: f64,
    pub is_in_call: bool,
    pub pending_deposits: f64,
    #[serde(default)]
    pub margin_balance: f64,
    #[serde(default)]
    pub short_balance: f64,
    pub account_value: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentBalances {
    pub accrued_interest: f64,
    pub cash_balance: f64,
    pub cash_receipts: f64,
    pub long_option_market_value: f64,
    pub liquidation_value: f64,
    pub long_market_value: f64,
    pub money_market_fund: f64,
    pub savings: f64,
    pub short_market_value: f64,
    pub pending_deposits: f64,
    pub available_funds: f64,
    pub available_funds_non_marginable_trade: f64,
    pub buying_power: f64,
    pub buying_power_non_marginable_trade: f64,
    pub day_trading_buying_power: f64,
    #[serde(default)]
    pub equity: f64,
    #[serde(default)]
    pub equity_percentage: f64,
    #[serde(default)]
    pub long_margin_value: f64,
    #[serde(default)]
    pub maintenance_call: f64,
    #[serde(default)]
    pub maintenance_requirement: f64,
    #[serde(default)]
    pub margin_balance: f64,
    #[serde(rename = "regTCall")]
    pub reg_tcall: f64,
    pub short_balance: f64,
    #[serde(default)]
    pub short_margin_value: f64,
    pub short_option_market_value: f64,
    pub sma: f64,
    pub mutual_fund_value: f64,
    pub bond_value: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectedBalances {
    pub available_funds: f64,
    pub available_funds_non_marginable_trade: f64,
    pub buying_power: f64,
    pub day_trading_buying_power: f64,
    pub day_trading_buying_power_call: f64,
    #[serde(default)]
    pub maintenance_call: f64,
    #[serde(rename = "regTCall")]
    pub reg_tcall: f64,
    pub is_in_call: bool,
    pub stock_buying_power: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderStrategy {
    pub session: String,
    pub duration: String,
    pub order_type: String,
    pub complex_order_strategy_type: String,
    pub quantity: f64,
    pub filled_quantity: f64,
    pub remaining_quantity: f64,
    pub requested_destination: String,
    pub destination_link_name: String,
    pub price: f64,
    pub order_leg_collection: Vec<OrderLegCollection>,
    pub order_strategy_type: String,
    pub order_id: i64,
    pub cancelable: bool,
    pub editable: bool,
    pub status: String,
    pub entered_time: String,
    pub close_time: String,
    pub account_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderLegCollection {
    pub order_leg_type: String,
    pub leg_id: i64,
    pub instrument: Instrument,
    pub instruction: String,
    pub position_effect: String,
    pub quantity: f64,
}
