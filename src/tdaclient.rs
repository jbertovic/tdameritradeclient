#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::must_use_candidate)]
use crate::param::{
    convert_to_pairs, Pair
};
use crate::request::Endpoint;
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
    auth_token: String,
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
            auth_token: token,
            client,
        }
    }
    ///
    /// change timeout configuration of Session
    ///
    pub fn session_timeout(&mut self, duration: Duration) {
        self.client.read_timeout(duration);
    }
    ///
    /// get endpoint with query parameters
    /// 
    /// See `response::Endpoint` for available Endpoints
    /// See param for matching parameters
    /// 
    pub fn get<'a, P, T>(&self, ep: &Endpoint, params: P) -> T 
        where
        RequestBuilder: Execute<T>,
        P: IntoIterator,
        P::Item: Pair<'a>,
    {
        self.client.get(ep.url_endpoint())
        .params(convert_to_pairs(params))
        .execute()
    }
    ///
    /// get /accounts/{account}/watchlists
    /// retrieves all watchlists for an account
    pub fn get_watchlists<T>(&self, account: &str) -> T
    where
        RequestBuilder: Execute<T>,
    {
        self.client
            .get(format!("{}accounts/{}/watchlists", crate::APIWWW, account))
            .execute()
    }
    ///
    /// Post /accounts/{account}/orders with JSON formated body
    /// Creates a working order
    /// if JSON body has error it will return json indicating what's wrong
    /// if nothing is returned than request was good - could add additional error checking for 201 or 200 response
    pub fn create_order(&self, account: &str, ordertxt: &str) -> String {
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
    pub fn delete_order<T>(&self, account: &str, order: &str) -> T
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
    pub fn replace_order(&self, account: &str, order: &str, ordertxt: &str) -> String {
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

