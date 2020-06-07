#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::must_use_candidate)]
static APIWWW: &str = "https://api.tdameritrade.com/v1/";
use attohttpc::{RequestBuilder, Session};
use crate::param::{History, OptionChain, Account, Order};

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
    /// Requires valid auth token from tdameritrade
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
        self.client
            .get(format!("{}userprincipals", APIWWW))
            .execute()
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
    pub fn gethistory<T>(&self, symbol: &str, params: &[History]) -> T
    where
        RequestBuilder: Execute<T>,
    {
        let mut builder = self
            .client
            .get(format!("{}marketdata/{}/pricehistory", APIWWW, symbol));
        for param in params {
            let (k, v) = param.into();
            builder = builder.param(k, v);
        }
        builder.execute()
    }
    /// get /marketdata/chains?symbol=SYM
    /// additional query parameters need to be added from `OptionChain` Enum
    pub fn getoptionchain<T>(&self, symbol: &str, params: &[OptionChain]) -> T
    where
        RequestBuilder: Execute<T>,
    {
        let mut builder = self
            .client
            .get(format!("{}marketdata/chains", APIWWW))
            .param("symbol", symbol);
        for param in params {
            let (k, v) = param.into();
            builder = builder.param(k, v);
        }
        builder.execute()
    }
    /// get /accounts
    /// if there are more than one account linked than it will retrieve an array of accounts
    pub fn getaccounts<T>(&self) -> T
    where
        RequestBuilder: Execute<T>,
    {
        self.client.get(format!("{}accounts", APIWWW)).execute()
    }
    /// get /accounts/{account}
    /// grabs one account with `account_id`
    /// additional query parameters need to be added from `Account` Enum
    pub fn getaccount<T>(&self, account: &str, params: &[Account]) -> T
    where
        RequestBuilder: Execute<T>,
    {
        let mut builder = self.client.get(format!("{}accounts/{}", APIWWW, account));
        for param in params {
            let (k, v) = param.into();
            builder = builder.param(k, v);
        }
        builder.execute()
    }
    /// get /accounts/{account}/orders
    /// retrieve all working orders
    pub fn getorders<T>(&self, account: &str, params: &[Order]) -> T
    where
        RequestBuilder: Execute<T>,
    {
        let mut builder = self.client.get(format!("{}accounts/{}/orders", APIWWW, account));
        for param in params {
            let (k, v) = param.into();
            builder = builder.param(k, v);
        }
        builder.execute()
    }
    /// Post /accounts/{account}/orders with JSON formated body
    /// Creates a working order
    /// if JSON body has error it will return json indicating what's wrong
    /// if nothing is returned than request was good - could add additional error checking for 201 or 200 response
    pub fn createorder(&self, account: &str, ordertxt: &str) -> String
    {
        self.client
            .post(format!("{}accounts/{}/orders", APIWWW, account))
            .header_append("Content-Type", "application/json")
            .text(ordertxt)
            .send()
            .expect("Trouble Retrieving Response: ERROR")
            .text().unwrap()
    }
    /// Delete /accounts/{account}/orders/{order}
    /// Creates a working order
    pub fn deleteorder(&self, account: &str, order: &str) -> String
    {
        self.client
            .delete(format!("{}accounts/{}/orders/{}", APIWWW, account, order))
            .send()
            .expect("Trouble Retrieving Response: ERROR")
            .text().unwrap()
    }

    /// PUT /accounts/{account}/orders/{order} with JSON formated body
    /// Replaces a working order with new order - allow the API to cancel and then creates new order
    pub fn replaceorder(&self, account: &str, order: &str, ordertxt: &str) -> String
    {
        self.client
            .put(format!("{}accounts/{}/orders/{}", APIWWW, account, order))
            .header_append("Content-Type", "application/json")
            .text(ordertxt)
            .send()
            .expect("Trouble Retrieving Response: ERROR")
            .text().unwrap()
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
mod tdaclient_tests{
    #[test]
    fn check_if_this_test_works() {
        assert!(true);
    }


}