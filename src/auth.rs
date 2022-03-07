use crate::model::token::{ErrorResponse, TokenResponse};
use log::info;
use serde::Serialize;
use std::time::SystemTime;

///
/// Convenience function
///
/// used to get a valid `token` from `refresh_token` and `clientid`
///
pub fn get_token_from_refresh(refresh: &str, clientid: &str) -> String {
    // create new TDauth struct using refresh / clientid
    // return token
    let newauth = TDauth::new_from_refresh(refresh, clientid, false);
    newauth.log_change("access token created from refresh");
    newauth.token
}
///
/// Convenience function
///
/// used to get a valid `refresh` from `refresh_token` and `clientid`
///
pub fn get_refresh_from_refresh(refresh: &str, clientid: &str) -> String {
    // create new TDauth struct using refresh / clientid
    // return token
    let newauth = TDauth::new_from_refresh(refresh, clientid, true);
    newauth.log_change("refresh token created from refresh");
    newauth.refresh
}
///
/// used to get a valid `token` and `refresh_token` from `code`, `clientid` and `redirecturi`
///
/// you can use decode=true if you did **NOT** decode it **only useful if you are using the browser to get code from query string**
///
pub fn get_refresh_from_code(
    code: &str,
    clientid: &str,
    redirecturi: &str,
    codedecode: bool,
) -> String {
    // create new TDauth struct using refresh / clientid
    // return token
    let newauth = TDauth::new_from_code(code, clientid, redirecturi, codedecode);
    newauth.refresh
}
///
/// used to get code manually from tdameritrade website with redirect URI as localhost
/// as per the directions on developer.tdameritrade.com -> you must register an app to get clientid and redirecturi
/// code can be used with `gettokenfromcode` to initiate a refreshtoken and token
///
pub fn get_code_weblink(clientid: &str, redirecturi: &str) -> String {
    let mut getcodeurl = url::Url::parse("https://auth.tdameritrade.com/auth").unwrap();
    getcodeurl
        .query_pairs_mut()
        .append_pair("response_type", "code")
        .append_pair("redirect_uri", redirecturi)
        .append_pair("client_id", &format!("{}{}", clientid, "@AMER.OAUTHAP"));
    getcodeurl.to_string()
}
///
/// These are tools to help manage authorization tokens with TD Ameritrade API
///
/// 1) `getcodeweblink` is a link created from parameters registered with deverloper.tdameritrade.com.  
/// The link can be used to return a code to use to request a valid token and refresh token.  You will need to log in with your TDAmeritrade Credentials.
/// If you use a `redirect_uri` that points to localhost than you will see the code returned in the query bar of the browser
/// 2) `new_fromcode` will allow you to update tokens from the code retrieved in 1)
/// 3) `new_fromrefresh` will allow you to update tokens from the `refresh_token`.  The `refresh_token` will stay active for 90 days so you can save for reuse.
///
#[derive(Debug, Clone, Default, Serialize)]
pub struct TDauth {
    token: String,
    refresh: String,
    client_id: String,
    redirect_uri: Option<String>,
    token_expire_epoch: u64,
    refresh_expire_epoch: u64,
    error: String,
}

impl TDauth {

    /// create new `TDauth` with configuration only
    /// 
    pub fn new(refresh: String, client_id: String, redirect_uri: String) -> Self {
        let mut newauth = TDauth::default();
        newauth.set_refresh(refresh);
        newauth.set_client_id(client_id);
        newauth.set_redirect_uri(redirect_uri);
        newauth
    }

    /// create new `TDauth` with `refresh_token` and `clientid`
    /// if successful `TDauth` will carry new valid `token`
    /// if refreshupdate is true than `refresh_token` will also be updated
    pub fn new_from_refresh(refresh: &str, client_id: &str, refresh_update: bool) -> Self {
        let mut newauth = TDauth::default();
        newauth.set_refresh(refresh.to_string());
        newauth.set_client_id(client_id.to_string());
        newauth.resolve_token_from_refresh(refresh_update);
        newauth.log_change("TDauth tokens created from refresh grant");
        newauth
    }
    /// create new `TDauth` with `code`, `redirecturi` and `clientid`
    /// if successful `TDauth` will carry both new `refresh_token` and new valid `token`
    ///
    /// you can use decode=true if you did **NOT** decode it **only useful if you are using the browser to get code from query string**
    ///
    pub fn new_from_code(
        code: &str,
        client_id: &str,
        redirect_uri: &str,
        code_decode: bool,
    ) -> Self {
        let mut newauth = TDauth::default();
        newauth.set_client_id(client_id.to_string());
        newauth.set_redirect_uri(redirect_uri.to_string());
        newauth.resolve_token_fromcode(code, code_decode);
        newauth.log_change("TDauth tokens created from code grant");
        newauth
    }
    /// get /oauth2/token
    /// token endpoint returns an access token along with an refresh token
    /// using `refresh_token` grant type and retrieves new `refresh_token` (optional)
    ///
    /// returns full response and updates `TDauth` struct
    ///
    pub fn resolve_token_from_refresh(&mut self, refresh_update: bool) {
        let refresh = self.refresh.clone();
        let client_id = format!("{}{}", self.client_id.clone(), crate::APIKEY);

        //body parameters
        let mut body = vec![
            ("grant_type", "refresh_token"),
            ("refresh_token", &refresh),
            ("client_id", &client_id),
        ];
        if refresh_update {
            body.push(("access_type", "offline"));
        }

        self.auth_request(body, refresh_update);
    }

    /// get /oauth2/token
    /// token endpoint returns an access token along with an optional refresh token
    /// using `authorization_code` grant type and retrieves new `refresh_token` (response returned) while storing valid token inside client
    ///
    /// returns full json response as text and updates TDAuth
    ///
    /// if grabbing code from browser as per the instructions on developer.tdameritrade.com
    /// then you will need to decode it.  As the code is encoded when put in post body.
    ///
    /// you can use decode=true if you did **NOT** decode it **only useful if you are using the browser to get code from query string**
    ///
    pub fn resolve_token_fromcode(&mut self, code: &str, codedecode: bool) {
        // is code already decoded or not? - ask did it come from a query parameter (codedecode=true) or some other way (codedecode=false)
        let decoded_code = if codedecode {
            let (_, decoded) = url::form_urlencoded::parse(format!("code={}", code).as_bytes())
                .into_owned()
                .next()
                .unwrap();
            decoded
        } else {
            code.to_owned()
        };

        let redirect_uri = self.redirect_uri.as_ref().unwrap().clone();
        let client_id = format!("{}{}", self.client_id.clone(), crate::APIKEY);

        //body parameters
        let body = vec![
            ("grant_type", "authorization_code"),
            ("access_type", "offline"),
            ("code", &decoded_code),
            ("client_id", &client_id),
            ("redirect_uri", &redirect_uri),
        ];

        self.auth_request(body, true);
    }

    fn auth_request(&mut self, body: Vec<(&str, &str)>, refresh_update: bool) {
        // any web issues
        let response = match request_auth(body) {
            Ok(r) => r,
            Err(e) => {
                self.error = e.to_string();
                self.reset_tokens();
                return;
            }
        };

        // any authorization issues
        if response.contains("\"error\" :") {
            let error_response: ErrorResponse = serde_json::from_str(&response).unwrap();
            self.error = format!("Error response from server: {}", error_response.error);
            self.reset_tokens();
            return;
        }

        // any parsing issues
        let token_response: TokenResponse = match serde_json::from_str(&response) {
            Ok(t) => t,
            Err(e) => {
                self.error = e.to_string();
                self.reset_tokens();
                return;
            }
        };

        let epoch = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        self.token = token_response.access_token;
        self.token_expire_epoch = token_response.expires_in + epoch;
        if refresh_update {
            self.refresh = token_response.refresh_token;
            self.refresh_expire_epoch = token_response.refresh_token_expires_in + epoch;
        }
    }

    pub fn is_token_valid(&self, buffer: u64) -> bool {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + buffer
            < self.token_expire_epoch
    }

    pub fn is_refresh_valid(&self, buffer: u64) -> bool {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + buffer
            < self.refresh_expire_epoch
    }

    pub fn get_tokens(&self) -> (&str, &str) {
        (&self.token, &self.refresh)
    }

    pub fn get_auth_token(&self) -> &str {
        &self.token
    }

    pub fn reset_expire(&mut self) {
        self.token_expire_epoch = 0;
        self.refresh_expire_epoch = 0;
    }

    fn reset_tokens(&mut self) {
        self.token = String::new();
        self.refresh = String::new();
        self.reset_expire();
    }

    fn set_refresh(&mut self, refresh: String) {
        self.refresh = refresh.to_owned();
    }

    fn set_client_id(&mut self, client_id: String) {
        self.client_id = client_id;
    }

    fn set_redirect_uri(&mut self, redirect_uri: String) {
        self.redirect_uri = Some(redirect_uri.to_owned());
    }

    pub fn log_change(&self, desc: &str) {
        if !self.error.is_empty() {
            info!("{}-Error: {}", desc, &self.error);
        } else {
            info!("{}", desc);
        }
    }

    pub fn web_link_authorization(&self) -> String {
        get_code_weblink(&self.client_id, self.redirect_uri.as_ref().unwrap())    
    }
}

fn request_auth(body: Vec<(&str, &str)>) -> Result<String, attohttpc::Error> {
    Ok(attohttpc::post(format!("{}oauth2/token", crate::APIWWW))
        .form(&body)?
        .send()?
        .text()?)
}

#[cfg(test)]
mod auth_tests {

    use super::TDauth;
    use std::env;

    #[test]
    #[ignore]
    fn check_code_weblink_auth_works() {
        let clientid = env::var("TDCLIENTKEY").unwrap();
        let redirecturi = env::var("TDREDIRECT").unwrap();
        println!("{}", crate::auth::get_code_weblink(&clientid, &redirecturi));
    }

    #[test]
    #[ignore]
    fn check_new_fromcode_constructs_tdauth() {
        let code = env::var("TDCODE").unwrap();
        let clientid = env::var("TDCLIENTKEY").unwrap();
        let redirecturi = env::var("TDREDIRECT").unwrap();
        let newtdauth = TDauth::new_from_code(&code, &clientid, &redirecturi, true);
        println!("{:?}", newtdauth);
    }

    #[test]
//    #[ignore]
    fn check_new_fromrefresh_constructs_tdauth() {
        let refresh = env::var("TDREFRESHTOKEN").unwrap();
        let clientid = env::var("TDCLIENTKEY").unwrap();
        let newtdauth = TDauth::new_from_refresh(&refresh, &clientid, true);
        let (t, r) = newtdauth.get_tokens();
        println!("token: {} \nrefresh: {} \n", t, r);
        println!("{:?}", newtdauth);
    }
}
