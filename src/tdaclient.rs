#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::must_use_candidate)]
static APIWWW: &str = "https://api.tdameritrade.com/v1/";
static APIKEY: &str = "@AMER.OAUTHAP";
use attohttpc::{RequestBuilder, Session};
use crate::param::{History, OptionChain, Account, Order};
///
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
    authtoken: String,
    client: Session,
}

impl TDAClient {
    /// Create new bsae client that maintains Authorization Header
    /// Requires valid ***token*** from tdameritrade
    pub fn new(token: String) -> TDAClient {
        let mut client = Session::new();
        client.header("AUTHORIZATION", format!("Bearer {}", &token));

        TDAClient {
            authtoken: token,
            client,
        }
    }
    /// Create new base client that maintains Authorization Header
    /// Requires valid ***refresh token*** from tdameritrade
    pub fn newusingrefresh(refresh: &str, clientid: &str) -> TDAClient {
        let mut client = TDAClient::new("notvalid".to_owned());
        client.gettokenfromrefresh(refresh, clientid, false);
        client
    }
    /// Create new base client that maintains Authorization Header
    /// Requires valid auth ***code*** from tdameritrade
    pub fn newusingcode(code: &str, clientid: &str, redirect: &str) -> TDAClient {
        unimplemented!();
    }
    /// get /oauth2/token
    /// token endpoint returns an access token along with an refresh token
    /// using `refresh_token` grant type and retrieves new `refresh_token` (optional) while storing valid token inside client
    /// returns full response
    pub fn gettokenfromrefresh(&mut self, refresh: &str, clientid: &str, refreshupdate: bool) -> String 
    {
        //create new Session since authorization will change
        self.client = Session::new();
        let fullclientid = format!("{}{}", clientid, APIKEY);
        //body parameters
        let mut p = vec!(("grant_type", "refresh_token"),
                ( "refresh_token", refresh),
                ("client_id", &fullclientid));
        if refreshupdate {p.push(("access_type", "offline"));}
        let response = self.client
            .post(format!("{}oauth2/token", APIWWW))
            .form(&p).unwrap()
            .send().unwrap()
            .text().unwrap();

        let responsejson: serde_json::Value = serde_json::from_str(&response).expect("Error: No access token retrieved");
        self.authtoken = responsejson["access_token"].as_str().unwrap().to_owned();
        self.client.header("AUTHORIZATION", format!("Bearer {}", &self.authtoken));
        response
    }
    /// used to get code manually from tdameritrade website with redirect URI as localhost
    /// as per the directions on developer.tdameritrade.com -> you must register an app to get clientid and redirecturi
    /// code can be used with `gettokensfromcode` to initiate a refreshtoken and token
    pub fn getcodeweblink(clientid: &str, redirecturi: &str) -> String {
        let mut getcodeurl  = url::Url::parse("https://auth.tdameritrade.com/auth").unwrap();
        getcodeurl.query_pairs_mut().append_pair("response_type", "code")
            .append_pair("redirect_uri", redirecturi)
            .append_pair("client_id", &format!("{}{}", clientid, "@AMER.OAUTHAP"));
        getcodeurl.to_string()
    }
    /// get /oauth2/token
    /// token endpoint returns an access token along with an optional refresh token
    /// using `authorization_code` grant type and retrieves new `refresh_token` (response returned) while storing valid token inside client
    /// returns full response
    pub fn gettokenfromcode(&self, _code: &str, _clientid: &str, _redirect_uri: &str) -> String
    {
        unimplemented!();
        //TODO: implement authorization_code grant_type
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

    use super::TDAClient;
    use std::env;

    #[test]
    fn check_if_this_test_works() {
        assert!(true);
    }
    #[test]
    #[ignore]
    fn check_if_auth_works_gettokenfromrefresh() {
        let mut c = TDAClient::new("OLDTOKENTHATDOESNTWORK".to_owned());
        let r = env::var("TDREFRESHTOKEN").unwrap();
        let ck = format!("{}{}", env::var("TDCLIENTKEY").unwrap(), "@AMER.OAUTHAP");
        println!("{}", c.gettokenfromrefresh(&r, &ck, true));
        println!("client: {:?}", c);
        println!("{}", c.getuserprincipals::<String>());
    }
    #[test]
    #[ignore]
    fn check_if_newusingrefresh_creates_new_client() {
        let r = env::var("TDREFRESHTOKEN").unwrap();
        let ck = format!("{}{}", env::var("TDCLIENTKEY").unwrap(), "@AMER.OAUTHAP");
        let c = TDAClient::newusingrefresh(&r, &ck);
        println!("{}", c.getuserprincipals::<String>());
    }
    #[test]
    #[ignore]
    fn check_code_weblink_auth_works() {
        let ck = env::var("TDCLIENTKEY").unwrap();
        let redirect = env::var("TDREDIRECT").unwrap();
        println!("{}",TDAClient::getcodeweblink(&ck, &redirect));
    }
    #[test]
    #[ignore]
    fn check_if_newusingcode_creates_new_client() {
        unimplemented!();
    }
    #[test]
    #[ignore]
    fn check_if_auth_works_gettokenfromcode() {
        unimplemented!();
    }

}