use serde::{Deserialize, Serialize};
///
/// Holds Type for UserPrincipals response
///
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPrincipals {
    #[serde(skip)]
    pub auth_token: String,
    pub user_id: String,
    pub user_cd_domain_id: String,
    pub primary_account_id: String,
    pub last_login_time: String,
    pub token_expiration_time: String,
    pub login_time: String,
    pub access_level: String,
    pub stale_password: bool,
    #[serde(default)]
    pub streamer_info: StreamerInfo,
    pub professional_status: String,
    pub quotes: Quotes,
    #[serde(default)]
    pub streamer_subscription_keys: StreamerSubscriptionKeys,
    pub accounts: Vec<Account>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamerInfo {
    pub streamer_binary_url: String,
    pub streamer_socket_url: String,
    pub token: String,
    pub token_timestamp: String,
    pub user_group: String,
    pub access_level: String,
    pub acl: String,
    pub app_id: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quotes {
    pub is_nyse_delayed: bool,
    pub is_nasdaq_delayed: bool,
    pub is_opra_delayed: bool,
    pub is_amex_delayed: bool,
    pub is_cme_delayed: bool,
    pub is_ice_delayed: bool,
    pub is_forex_delayed: bool,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamerSubscriptionKeys {
    pub keys: Vec<Key>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Key {
    pub key: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub account_id: String,
    #[serde(default)]
    pub description: String,
    pub display_name: String,
    pub account_cd_domain_id: String,
    pub company: String,
    pub segment: String,
    #[serde(default)]
    pub surrogate_ids: String,
    #[serde(default)]
    pub preferences: Preferences,
    pub acl: String,
    pub authorizations: Authorizations,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Preferences {
    pub express_trading: bool,
    pub direct_options_routing: bool,
    pub direct_equity_routing: bool,
    pub default_equity_order_leg_instruction: String,
    pub default_equity_order_type: String,
    pub default_equity_order_price_link_type: String,
    pub default_equity_order_duration: String,
    pub default_equity_order_market_session: String,
    pub default_equity_quantity: i64,
    pub mutual_fund_tax_lot_method: String,
    pub option_tax_lot_method: String,
    pub equity_tax_lot_method: String,
    pub default_advanced_tool_launch: String,
    pub auth_token_timeout: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Authorizations {
    pub apex: bool,
    pub level_two_quotes: bool,
    pub stock_trading: bool,
    pub margin_trading: bool,
    pub streaming_news: bool,
    pub option_trading_level: String,
    pub streamer_access: bool,
    pub advanced_margin: bool,
    pub scottrade_account: bool,
}
