///
/// used to get a valid `token` from `refresh_token` and `clientid`
///
pub fn gettoken_fromrefresh(refresh: &str, clientid: &str) -> String {
    // create new TDauth struct using refresh / clientid
    // return token
    let newauth = TDauth::new_fromrefresh(refresh, clientid);
    newauth.token
}
///
/// used to get a valid `token` and `refresh_token` from `code`, `clientid` and `redirecturi`
///
/// you can use decode=true if you did **NOT** decode it **only useful if you are using the browser to get code from query string**
///
pub fn gettoken_fromcode(code: &str, clientid: &str, redirecturi: &str, codedecode: bool) -> String {
    // create new TDauth struct using refresh / clientid
    // return token
    let newauth = TDauth::new_fromcode(code, clientid, redirecturi, codedecode);
    newauth.token
}
///
/// used to get code manually from tdameritrade website with redirect URI as localhost
/// as per the directions on developer.tdameritrade.com -> you must register an app to get clientid and redirecturi
/// code can be used with `gettokenfromcode` to initiate a refreshtoken and token
///
pub fn getcodeweblink(clientid: &str, redirecturi: &str) -> String {
    let mut getcodeurl = url::Url::parse("https://auth.tdameritrade.com/auth").unwrap();
    getcodeurl
        .query_pairs_mut()
        .append_pair("response_type", "code")
        .append_pair("redirect_uri", redirecturi)
        .append_pair("client_id", &format!("{}{}", clientid, "@AMER.OAUTHAP"));
    getcodeurl.to_string()
}

#[derive(Debug)]
struct TDauth {
    token: String,
    refresh: String,
    clientid: String,
    redirecturi: Option<String>,
}

impl TDauth {
    /// retrieve valid token from refresh and clientid
    fn new_fromrefresh(refresh: &str, clientid: &str) -> TDauth {
        let mut newauth = TDauth {
            token: String::new(),
            refresh: refresh.to_owned(),
            clientid: format!("{}{}", clientid, crate::APIKEY),
            redirecturi: None,
        };
        newauth.resolve_token_fromrefresh(false);
        newauth
    }
    /// retrieve valid token from refresh and clientid
    ///
    /// you can use decode=true if you did **NOT** decode it **only useful if you are using the browser to get code from query string**
    ///
    fn new_fromcode(code: &str, clientid: &str, redirecturi: &str, codedecode: bool) -> TDauth {
        let mut newauth = TDauth {
            token: String::new(),
            refresh: String::new(),
            clientid: format!("{}{}", clientid, crate::APIKEY),
            redirecturi: Some(redirecturi.to_owned()),
        };
        newauth.resolve_token_fromcode(code, codedecode);
        newauth
    }
    /// get /oauth2/token
    /// token endpoint returns an access token along with an refresh token
    /// using `refresh_token` grant type and retrieves new `refresh_token` (optional)
    /// returns full response and updates `TDauth` struct
    pub fn resolve_token_fromrefresh(&mut self, refreshupdate: bool) -> String {
        //body parameters
        let mut p = vec![
            ("grant_type", "refresh_token"),
            ("refresh_token", &self.refresh),
            ("client_id", &self.clientid),
        ];
        if refreshupdate {
            p.push(("access_type", "offline"));
        }
        let response = attohttpc::post(format!("{}oauth2/token", crate::APIWWW))
            .form(&p)
            .unwrap()
            .send()
            .unwrap()
            .text()
            .unwrap();

        let responsejson: serde_json::Value =
            serde_json::from_str(&response).expect("Error: No access token retrieved");
        self.token = responsejson["access_token"].as_str().unwrap().to_owned();
        response
    }
    /// get /oauth2/token
    /// token endpoint returns an access token along with an optional refresh token
    /// using `authorization_code` grant type and retrieves new `refresh_token` (response returned) while storing valid token inside client
    /// returns full response
    ///
    /// if grabbing code from browser as per the instructions on developer.tdameritrade.com
    /// then you will need to decode it.  As the code is encoded when put in post body.
    ///
    /// you can use decode=true if you did **NOT** decode it **only useful if you are using the browser to get code from query string**
    ///
    pub fn resolve_token_fromcode(&mut self, code: &str, codedecode: bool) -> String {

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

        //body parameters
        let p = vec![
            ("grant_type", "authorization_code"),
            ("access_type", "offline"),
            ("code", &decoded_code),
            ("client_id", &self.clientid),
            ("redirect_uri", self.redirecturi.as_ref().unwrap()),
        ];

        let response = attohttpc::post(format!("{}oauth2/token", crate::APIWWW))
            .form(&p)
            .unwrap()
            .send()
            .unwrap()
            .text()
            .unwrap();

        let responsejson: serde_json::Value =
            serde_json::from_str(&response).expect("Error: No access token retrieved");
        self.token = responsejson["access_token"].as_str().unwrap().to_owned();
        self.refresh = responsejson["refresh_token"].as_str().unwrap().to_owned();
        response
    }
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
        println!("{}", crate::auth::getcodeweblink(&clientid, &redirecturi));
    }

    #[test]
    #[ignore]
    fn check_new_fromcode_constructs_tdauth() {
        let code = env::var("TDCODE").unwrap();
        let clientid = env::var("TDCLIENTKEY").unwrap();
        let redirecturi = env::var("TDREDIRECT").unwrap();
        let newtdauth = TDauth::new_fromcode(&code, &clientid, &redirecturi, true);
        println!("{:?}", newtdauth);
    }

    #[test]
    #[ignore]
    fn check_new_fromrefresh_constructs_tdauth() {
        let refresh = env::var("TDREFRESHTOKEN").unwrap();
        let clientid = env::var("TDCLIENTKEY").unwrap();
        let newtdauth = TDauth::new_fromrefresh(&refresh, &clientid);
        println!("{:?}", newtdauth);
    }
}
