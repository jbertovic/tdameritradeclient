#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::must_use_candidate)]
static APIWWW: &str = "https://api.tdameritrade.com/v1/";
use attohttpc::{RequestBuilder, Session};
/// # TDA Client
///
/// Uses `attohttpc::RequestBuilder` to build requests and `attohttpc::Session` to maintain the same client configuration
///
/// Two options for output:
/// 1) text which in this case is JSON from TDA API
/// 2) convert to `serde_json::Value`
///

#[derive(Debug)]
pub struct TDAClient {
    // consumerkey: String,
    authtoken: String,
    client: Session,
}

#[allow(dead_code)]
impl TDAClient {
    /// Create new bsae client that maintains Authorization Header
    pub fn new(token: String) -> TDAClient {
        let mut client = Session::new();
        client.header("AUTHORIZATION", format!("Bearer {}", &token));

        TDAClient {
            authtoken: token,
            client,
        }
    }
    /// get /userprincipals
    pub fn getuserprincipals<T>(&self) -> T
    where
        RequestBuilder: Execute<T>,
    {
        self.client.get(format!("{}userprincipals", APIWWW)).execute()
    }
    /// get /marketdata/quotes?symbol=SYM1,SYM2,SYM3....
    pub fn getquotes<T>(&self, quotequery: &str) -> T
    where
        RequestBuilder: Execute<T>,
    {
        self.client
            .get(format!("{}marketdata/quotes", APIWWW))
            .param("symbol", quotequery)
            .execute()
    }
    /// get /marketdata/{SYM}/pricehistory
    /// additional query parameters need to be added from `History` Enum
    /// retrieved based on EPOCH datetime
    /// also `History` Enum starttime and endtime is in EPOCH
    pub fn gethistory(&self, symbol: &str) -> RequestBuilder {
        self.client
            .get(format!("{}marketdata/{}/pricehistory", APIWWW, symbol))
    }
    /// get /marketdata/chains?symbol=SYM
    /// additional query parameters need to be added from `OptionChain` Enum
    pub fn getoptionchain(&self, symbol: &str) -> RequestBuilder {
        self.client
            .get(format!("{}marketdata/chains", APIWWW))
            .param("symbol", symbol)
    }
    /// get /accounts
    pub fn getaccounts<T>(&self) -> T
    where
        RequestBuilder: Execute<T>,
    {
        self.client.get(format!("{}accounts", APIWWW)).execute()
    }
    /// get /accounts/{account}
    /// additional query parameters need to be added from `Account` Enum
    pub fn getaccount(&self, account: &str) -> RequestBuilder {
        self.client.get(format!("{}accounts/{}", APIWWW, account))
    }
    /// get /accounts/{account}/orders
    pub fn getorders<T>(&self, account: &str) -> T 
    where
        RequestBuilder: Execute<T>,
    {
        self.client
            .get(format!("{}accounts/{}/orders", APIWWW, account)).execute()
    }
    /// get /accounts/{account}/savedorders
    pub fn getsavedorders<T>(&self, account: &str) -> T 
    where
        RequestBuilder: Execute<T>,
    {
        self.client
            .get(format!("{}accounts/{}/savedorders", APIWWW, account)).execute()
    }
}
#[derive(Debug)]
pub enum Account {
    Positions,
    Orders,
    PositionsAndOrders,
}

impl Into<(&'static str, String)> for Account {
    fn into(self) -> (&'static str, String) {
        match self {
            Account::Positions => ("fields", String::from("positions")),
            Account::Orders => ("fields", String::from("orders")),
            Account::PositionsAndOrders => ("fields", String::from("positions,orders")),
        }
    }    
}

#[derive(Debug)]
pub enum History<'a> {
    PeriodType(&'a str),
    Period(u8),
    FrequencyType(&'a str),
    Frequency(u8),
    StartDate(u64),
    EndDate(u64),
    NeedExendedHoursData(bool),
}

impl<'a> Into<(&'static str, String)> for History<'a> {
    fn into(self) -> (&'static str, String) {
        match self {
            History::PeriodType(s) => ("periodType", s.into()),
            History::Period(i) => ("period", i.to_string()),
            History::FrequencyType(s) => ("frequencyType", s.into()),
            History::Frequency(i) => ("frequency", i.to_string()),
            History::StartDate(i) => ("startDate", i.to_string()),
            History::EndDate(i) => ("endDate", i.to_string()),
            History::NeedExendedHoursData(b) => ("needExtendedHoursData", b.to_string()),
        }
    }
}
#[derive(Debug)]
pub enum OptionChain<'a> {
    ContractType(&'a str),
    StrikeCount(u8),
    Strategy(&'a str),
    Interval(u8),
    Strike(f64),
    IncludeQuotes(bool),
    Range(&'a str),
    FromDate(&'a str),
    ToDate(&'a str),
    Volatility(f64),
    UnderlyingPrice(f64),
    InterestRate(f64),
    DaysToExpiration(f64),
    ExpireMonth(&'a str),
    OptionType(&'a str),
}

impl<'a> Into<(&'static str, String)> for OptionChain<'a> {
    fn into(self) -> (&'static str, String) {
        match self {
            OptionChain::ContractType(i) => ("contractType", i.to_string()),
            OptionChain::Strategy(s) => ("strategy", s.into()),
            OptionChain::StrikeCount(i) => ("strikeCount", i.to_string()),
            OptionChain::Interval(i) => ("interval", i.to_string()),
            OptionChain::Strike(i) => ("strike", i.to_string()),
            OptionChain::IncludeQuotes(b) => ("includeQuotes", b.to_string()),
            OptionChain::Range(s) => ("range", s.into()),
            OptionChain::FromDate(s) => ("fromDate", s.into()),
            OptionChain::ToDate(s) => ("toDate", s.into()),
            OptionChain::Volatility(i) => ("volatility", i.to_string()),
            OptionChain::UnderlyingPrice(i) => ("underlyingPrice", i.to_string()),
            OptionChain::InterestRate(i) => ("interestRate", i.to_string()),
            OptionChain::DaysToExpiration(i) => ("daysToExpiration", i.to_string()),
            OptionChain::ExpireMonth(s) => ("expireMonth", s.into()),
            OptionChain::OptionType(s) => ("optionType", s.into()),
        }
    }
}

// TODO: Execute should return a result to propogate error upward
pub trait Execute<T> {
    fn execute(self) -> T;
}

impl Execute<String> for RequestBuilder {
    fn execute(self) -> String {
        self.send()
            .expect("Trouble Retrieving Response: ERROR")
            .text()
            .expect("Response did not return JSON: ERROR")
    }
}

impl Execute<serde_json::Value> for RequestBuilder {
    fn execute(self) -> serde_json::Value {
        serde_json::from_str(
            self.send()
                .expect("Trouble Retrieving Response: ERROR")
                .text()
                .expect("Response did not return JSON: ERROR")
                .as_str(),
        )
        .expect("SERDE: Trouble parsing json text: ERROR")
    }
}

#[cfg(test)]
mod tests_tdaclient {

    // Tests should have some sort of mock to retrieve example json
    // These are more like examples
    // REQUIRES an active TD ameritrade account and valid token

    use super::*;
    use std::env;

    fn initialize_client() -> TDAClient {
        TDAClient::new(env::var("TDAUTHTOKEN").unwrap())
    }

    #[test]
    fn able_to_retrieve_user_data() {
        let resptxt: String = initialize_client().getuserprincipals();
        println!("{:?}", resptxt);
        assert_eq!(resptxt.starts_with("{\n  \"authToken\""), true);
    }

    #[test]
    fn able_to_retrieve_quotes() {
        let resptxt: String = initialize_client().getquotes("F,INTC,TRP");
        println!("{:?}", resptxt);
        assert_eq!(resptxt.contains("\"assetType\""), true);
    }

    #[test]
    fn able_to_retrieve_tojson() {
        let resptxt: serde_json::Value = initialize_client().getuserprincipals();
        println!("{:?}", resptxt);
        assert!(resptxt["userId"].is_string());
    }

    #[test]
    fn able_to_retrieve_history() {
        let resptxt: String = initialize_client()
            .gethistory("TRP")
            .params(
                &[History::Period(1).into(),
                History::PeriodType("month").into(),
                History::Frequency(1).into(),
                History::FrequencyType("daily").into()])
            .execute();
        println!("{:?}", resptxt);
        assert_eq!(resptxt.contains("\"candles\""), true);
    }

    #[test]
    fn able_to_retrieve_optionchain() {
        let resptxt: String = initialize_client()
            .getoptionchain("TRP")
            .params(&[OptionChain::StrikeCount(3).into()])
            .params(&[OptionChain::ContractType("CALL").into()])
            .execute();
        println!("{:?}", resptxt);
        assert_eq!(resptxt.contains("\"SUCCESS\""), true);
    }

    #[test]
    fn able_to_retrieve_all_accounts() {
        let resptxt: String = initialize_client().getaccounts();
        println!("{:?}", resptxt);
        assert_eq!(resptxt.contains("\"securitiesAccount\""), true);
    }

    #[test]
    fn able_to_retrieve_one_account() {
        let c = initialize_client();
        let user: serde_json::Value = c.getuserprincipals();
        let resptxt: String = c
            .getaccount(user["primaryAccountId"].as_str().unwrap())
            .execute();
        println!("{:?}", resptxt);
        assert_eq!(resptxt.contains("\"securitiesAccount\""), true);
    }

    #[test]
    fn able_to_retrieve_savedorders() {
        let c = initialize_client();
        let user: serde_json::Value = c.getuserprincipals();
        let resptxt: String = c.getsavedorders(user["primaryAccountId"].as_str().unwrap());
        println!("{:?}", resptxt);
        assert_eq!(resptxt.contains("\"orders\""), true);
    }

    #[test]
    fn able_to_retrieve_account_positions() {
        let c = initialize_client();
        let user: serde_json::Value = c.getuserprincipals();
        //let (k, v) = Account::Positions.into();
        let resptxt: String = c
            .getaccount(user["primaryAccountId"].as_str().unwrap())
            //.param(k, v)
            .params(&[Account::Positions.into()])
            .execute();
        println!("{:?}", resptxt);
        assert_eq!(resptxt.contains("\"positions\""), true);
    }
}
