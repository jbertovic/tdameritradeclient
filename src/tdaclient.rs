#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::must_use_candidate)]
use crate::param::{convert_to_pairs, Pair};
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
/// 3) use `serde_json::Value` output to parse into a response `model` type
///
#[derive(Debug, Default)]
pub struct TDAClient {
    auth_token: String,
    client: Session,
}

impl Clone for TDAClient {
    fn clone(&self) -> Self {
        TDAClient::new(self.auth_token.clone())
    }
}

impl TDAClient {
    ///
    /// Create new base client that maintains Authorization Header
    ///
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
    /// See `Endpoint` for available Endpoints
    ///
    /// See param for matching parameters
    ///
    pub fn get<'a, P, T>(&self, ep: &Endpoint, params: P) -> T
    where
        RequestBuilder: Execute<T>,
        P: IntoIterator,
        P::Item: Pair<'a>,
    {
        self.client
            .get(ep.url_endpoint())
            .params(convert_to_pairs(params))
            .execute()
    }

    ///
    /// post endpoint with json body
    ///
    /// # Errors
    /// if nothing was returned than request was good, otherwise a json string will be returned indicating error
    ///
    pub fn post<'a, T>(&self, ep: &Endpoint, body: &'a str) -> T
    where
        // RequestBuilder: Execute<T>,
        RequestBuilder<attohttpc::body::Text<&'a str>>: Execute<T>,
    {
        self.client
            .post(ep.url_endpoint())
            .header_append("Content-Type", "application/json")
            .text(body)
            .execute()
    }
    ///
    /// put endpoint with json body
    ///
    /// # Errors
    /// if nothing was returned than request was good, otherwise a json string will be returned indicating error
    ///
    pub fn put<'a, T>(&self, ep: &Endpoint, body: &'a str) -> T
    where
        // RequestBuilder: Execute<T>,
        RequestBuilder<attohttpc::body::Text<&'a str>>: Execute<T>,
    {
        self.client
            .put(ep.url_endpoint())
            .header_append("Content-Type", "application/json")
            .text(body)
            .execute()
    }
    ///
    /// patch endpoint with json body
    ///
    /// # Errors
    /// if nothing was returned than request was good, otherwise a json string will be returned indicating error
    ///
    pub fn patch<'a, T>(&self, ep: &Endpoint, body: &'a str) -> T
    where
        // RequestBuilder: Execute<T>,
        RequestBuilder<attohttpc::body::Text<&'a str>>: Execute<T>,
    {
        self.client
            .patch(ep.url_endpoint())
            .header_append("Content-Type", "application/json")
            .text(body)
            .execute()
    }

    ///
    /// delete endpoint
    ///
    pub fn delete<T>(&self, ep: &Endpoint) -> T
    where
        RequestBuilder: Execute<T>,
    {
        self.client.delete(ep.url_endpoint()).execute()
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

impl Execute<String> for RequestBuilder<attohttpc::body::Text<&str>> {
    fn execute(self) -> String {
        let response = preexecute_wbody(self);
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

/// created to help with logging
fn preexecute_wbody(req: RequestBuilder<attohttpc::body::Text<&str>>) -> Response {
    let mut prepared = req.prepare();
    info!(
        "Request: {}-{} - includes body text",
        prepared.method(),
        prepared.url()
    );
    let response = prepared.send().expect("Trouble Retrieving Response: ERROR");
    info!("Response: Status:{}", response.status());
    response
}
