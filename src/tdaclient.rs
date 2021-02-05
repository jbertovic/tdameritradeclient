#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::must_use_candidate)]
use crate::auth::{gettoken_fromcode, gettoken_fromrefresh};
use crate::param::{
    convert_to_pairs, Account, History, Instruments, OptionChain, Order, Transactions,
};
use attohttpc::{RequestBuilder, Response, Session};
use log::info;
use std::time::Duration;
///
/// Main client to access TD Ameritrade endpoints
///
/// Uses `attohttpc::RequestBuilder` to build requests and `attohttpc::Session` to maintain the same client configuration
///
/// Two options for output:
/// 1) text which in this case is JSON from TDA API
/// 2) convert to `serde_json::Value`
///
#[derive(Debug)]
pub struct TDAClient {
    authtoken: String,
    client: Session,
}

impl TDAClient {
    ///
    /// Create new base client that maintains Authorization Header
    /// Requires valid ***token*** from tdameritrade
    ///
    pub fn new(token: String) -> TDAClient {
        let mut client = Session::new();
        info!("New Client initialized - from token");
        client.header("AUTHORIZATION", format!("Bearer {}", &token));
        TDAClient {
            authtoken: token,
            client,
        }
    }
    ///
    /// Create new base client that maintains Authorization Header
    /// Requires valid ***refresh token*** from tdameritrade
    ///
    pub fn new_usingrefresh(refresh: &str, clientid: &str) -> TDAClient {
        info!("New Client initialized - from refresh token");
        TDAClient::new(gettoken_fromrefresh(refresh, clientid))
    }
    ///
    /// Create new base client that maintains Authorization Header
    /// Requires valid auth ***code*** from tdameritrade
    ///
    /// you can use decode=true if you did **NOT** decode it **only useful if you are using the browser to get code from query string**
    ///
    pub fn new_usingcode(
        code: &str,
        clientid: &str,
        redirecturi: &str,
        codedecode: bool,
    ) -> TDAClient {
        info!("New Client initialized - from authorization code");
        TDAClient::new(gettoken_fromcode(code, clientid, redirecturi, codedecode))
    }
    ///
    /// change timeout configuration of Session
    ///
    pub fn sesion_timeout(&mut self, duration: Duration) {
        self.client.read_timeout(duration);
    }
    ///
    /// get /userprincipals
    ///
    pub fn getuserprincipals<T>(&self) -> T
    where
        RequestBuilder: Execute<T>,
    {
        self.client
            .get(format!("{}userprincipals", crate::APIWWW))
            .execute()
    }
    ///
    /// get /marketdata/quotes?symbol=SYM1,SYM2,SYM3....
    pub fn getquotes<T>(&self, quotequery: &str) -> T
    where
        RequestBuilder: Execute<T>,
    {
        self.client
            .get(format!("{}marketdata/quotes", crate::APIWWW))
            .param("symbol", quotequery)
            .execute()
    }
    ///
    /// get /marketdata/{MARKET}/hours
    /// retrieve todays market hours for given market
    pub fn get_todays_market_hours<T>(&self, market: &str) -> T
    where
        RequestBuilder: Execute<T>,
    {
        self.client
            .get(format!("{}marketdata/{}/hours", crate::APIWWW, market))
            .execute()
    }
    ///
    /// get /marketdata/{MARKET}/hours
    /// retrieve market hours for given market and date
    pub fn get_dates_market_hours<T>(&self, market: &str, date: &str) -> T
    where
        RequestBuilder: Execute<T>,
    {
        self.client
            .get(format!("{}marketdata/{}/hours", crate::APIWWW, market))
            .param("date", date)
            .execute()
    }
    ///
    /// get /instruments
    ///
    /// Search or retrieve instrument data, including fundamental data.
    ///
    pub fn getinstruments<T>(&self, params: &[Instruments]) -> T
    where
        RequestBuilder: Execute<T>,
    {
        self.client
            .get(format!("{}instruments", crate::APIWWW))
            .params(convert_to_pairs(params))
            .execute()
    }
    ///
    /// get /instruments/cusip
    ///
    /// Get an instrument by CUSIP
    ///
    pub fn getinstrument<T>(&self, cusip: &str) -> T
    where
        RequestBuilder: Execute<T>,
    {
        self.client
            .get(format!("{}instruments/{}", crate::APIWWW, cusip))
            .execute()
    }
    ///
    /// get /marketdata/{SYM}/pricehistory
    /// additional query parameters need to be added from `History` Enum
    /// retrieved based on EPOCH datetime
    /// also `History` Enum starttime and endtime is in EPOCH
    pub fn gethistory<T>(&self, symbol: &str, params: &[History]) -> T
    where
        RequestBuilder: Execute<T>,
    {
        self.client
            .get(format!(
                "{}marketdata/{}/pricehistory",
                crate::APIWWW,
                symbol
            ))
            .params(convert_to_pairs(params))
            .execute()
    }
    ///
    /// get /marketdata/chains?symbol=SYM
    /// additional query parameters need to be added from `OptionChain` Enum
    pub fn getoptionchain<T>(&self, params: &[OptionChain]) -> T
    where
        RequestBuilder: Execute<T>,
    {
        self.client
            .get(format!("{}marketdata/chains", crate::APIWWW))
            .params(convert_to_pairs(params))
            .execute()
    }
    ///
    /// get /accounts
    /// if there are more than one account linked than it will retrieve an array of accounts
    pub fn getaccounts<T>(&self) -> T
    where
        RequestBuilder: Execute<T>,
    {
        self.client
            .get(format!("{}accounts", crate::APIWWW))
            .execute()
    }
    ///
    /// get /accounts/{account}
    /// grabs one account's balances with `account_id`
    /// additional query parameters need to be added from `Account` Enum
    pub fn getaccount<T>(&self, account: &str) -> T
    where
        RequestBuilder: Execute<T>,
    {
        let mut builder = self.client.get(format!("{}accounts/{}", crate::APIWWW, account)).execute()
    }
    ///
    /// get /accounts/{account}/positions
    /// grabs one accounts positions with `account_id`
    pub fn getpositions<T>(&self, account: &str) -> T
    where
        RequestBuilder: Execute<T>,
    {
        let mut builder = self.client.get(format!("{}accounts/{}/positions", crate::APIWWW, account)).execute()
    }
    ///
    /// get /accounts/{account}/orders
    /// retrieve all working orders
    pub fn getorders<T>(&self, account: &str, params: &[Order]) -> T
    where
        RequestBuilder: Execute<T>,
    {
        let mut builder = self.client.get(format!("{}accounts/{}/orders", crate::APIWWW, account));
        for param in params {
            let (k, v) = param.into();
            builder = builder.param(k, v);
        }
        builder.execute()
    }
    ///
    /// get /accounts/{account}/transactions
    /// retrieve a specified transaction by Id
    pub fn gettransactions<T>(&self, account: &str, params: &[Transactions]) -> T
    where
        RequestBuilder: Execute<T>,
    {
        self.client
            .get(format!(
                "{}accounts/{}/transactions",
                crate::APIWWW,
                account
            ))
            .params(convert_to_pairs(params))
            .execute()
    }
    ///
    /// get /accounts/{account}/transactions/{transactionId}
    /// retrieve a specified transaction by Id
    pub fn gettransaction<T>(&self, account: &str, transaction: &str) -> T
    where
        RequestBuilder: Execute<T>,
    {
        self.client
            .get(format!(
                "{}accounts/{}/transactions/{}",
                crate::APIWWW,
                account,
                transaction
            ))
            .execute()
    }
    ///
    /// get /accounts/{account}/watchlists
    /// retrieves all watchlists for an account
    pub fn get_watchlists<T>(&self, account: &str) -> T
    where
        RequestBuilder:Execute<T>,
    {
        self.client
            .get(format!(
                "{}accounts/{}/watchlists",
                crate::APIWWW,
                account
            ))
            .execute()
    }
    ///
    /// Post /accounts/{account}/orders with JSON formated body
    /// Creates a working order
    /// if JSON body has error it will return json indicating what's wrong
    /// if nothing is returned than request was good - could add additional error checking for 201 or 200 response
    pub fn createorder(&self, account: &str, ordertxt: &str) -> String {
        self.client
            .post(format!("{}accounts/{}/orders", crate::APIWWW, account))
            .header_append("Content-Type", "application/json")
            .text(ordertxt)
            .send()
            .expect("Trouble Retrieving Response: ERROR")
            .text()
            .unwrap()
    }
    ///
    /// Delete /accounts/{account}/orders/{order}
    /// Creates a working order
    pub fn deleteorder<T>(&self, account: &str, order: &str) -> T
    where
        RequestBuilder: Execute<T>,
    {
        self.client
            .delete(format!(
                "{}accounts/{}/orders/{}",
                crate::APIWWW,
                account,
                order
            ))
            .execute()
    }
    ///
    /// PUT /accounts/{account}/orders/{order} with JSON formated body
    /// Replaces a working order with new order - allow the API to cancel and then creates new order
    pub fn replaceorder(&self, account: &str, order: &str, ordertxt: &str) -> String {
        self.client
            .put(format!(
                "{}accounts/{}/orders/{}",
                crate::APIWWW,
                account,
                order
            ))
            .header_append("Content-Type", "application/json")
            .text(ordertxt)
            .send()
            .expect("Trouble Retrieving Response: ERROR")
            .text()
            .unwrap()
    }
}

/// This isn't called directly as its built into the functions of the `TDAClient`
///
/// Sends formed request to be executed with a return to either
/// 1) `String` - as text format
/// 2) `serde_json::Value` - as a JSON object format
///
pub trait Execute<T> {
    fn execute(self) -> T;
}

impl Execute<String> for RequestBuilder {
    fn execute(self) -> String {
        let response = preexecute(self);
        response
            .text()
            .expect("Response did not return BODY: ERROR")
    }
}

impl Execute<serde_json::Value> for RequestBuilder {
    fn execute(self) -> serde_json::Value {
        let response = preexecute(self);
        serde_json::from_str(
            response
                .text()
                .expect("Response did not return JSON: ERROR")
                .as_str(),
        )
        .expect("SERDE: Trouble parsing json text: ERROR")
    }
}

/// created to help with logging
fn preexecute(req: RequestBuilder) -> Response {
    let mut prepared = req.prepare();
    info!("Request: {}-{}", prepared.method(), prepared.url());
    let response = prepared.send().expect("Trouble Retrieving Response: ERROR");
    info!("Response: Status:{}", response.status());
    response
}

#[cfg(test)]
mod tdaclient_tests {

    use super::TDAClient;
    use std::env;

    #[test]
    #[ignore]
    fn check_new_usingrefresh_creates_new_client() {
        let refresh = env::var("TDREFRESHTOKEN").unwrap();
        let clientid = env::var("TDCLIENTKEY").unwrap();
        let c = TDAClient::new_usingrefresh(&refresh, &clientid);
        println!("{}", c.getuserprincipals::<String>());
    }

    #[test]
    #[ignore]
    fn check_if_newusingcode_creates_new_client() {
        let code = env::var("TDCODE").unwrap();
        let clientid = env::var("TDCLIENTKEY").unwrap();
        let redirecturi = env::var("TDREDIRECT").unwrap();
        let c = TDAClient::new_usingcode(&code, &clientid, &redirecturi, true);
        println!("{}", c.getuserprincipals::<String>());
    }
}
