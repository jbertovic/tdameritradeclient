static T_ERR: &str = "Error: Response returned no access token.  Check input parameters";
static R_ERR: &str = "Error: Trouble making request and parsing response";
///
/// used to get a valid `token` from `refresh_token` and `clientid`
///
pub fn gettoken_fromrefresh(refresh: &str, clientid: &str) -> String {
    // create new TDauth struct using refresh / clientid
    // return token
    let newauth = TDauth::new_fromrefresh(refresh, clientid, false);
    newauth.token
}
///
/// used to get a valid `refresh` from `refresh_token` and `clientid`
///
pub fn getrefresh_fromrefresh(refresh: &str, clientid: &str) -> String {
    // create new TDauth struct using refresh / clientid
    // return token
    let newauth = TDauth::new_fromrefresh(refresh, clientid, true);
    newauth.refresh
}
///
/// used to get a valid `token` and `refresh_token` from `code`, `clientid` and `redirecturi`
///
/// you can use decode=true if you did **NOT** decode it **only useful if you are using the browser to get code from query string**
///
pub fn gettoken_fromcode(
    code: &str,
    clientid: &str,
    redirecturi: &str,
    codedecode: bool,
) -> String {
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
///
/// These are tools to help manage authorization tokens with TD Ameritrade API
///
/// 1) `getcodeweblink` is a link created from parameters registered with deverloper.tdameritrade.com.  
/// The link can be used to return a code to use to request a valid token and refresh token.  You will need to log in with your TDAmeritrade Credentials.
/// If you use a `redirect_uri` that points to localhost than you will see the code returned in the query bar of the browser
/// 2) `new_fromcode` will allow you to update tokens from the code retrieved in 1)
/// 3) `new_fromrefresh` will allow you to update tokens from the `refresh_token`.  The `refresh_token` will stay active for 90 days so you can save for reuse.
///
#[derive(Debug)]
pub struct TDauth {
    token: String,
    refresh: String,
    clientid: String,
    redirecturi: Option<String>,
}

impl TDauth {
    /// create new `TDauth` with `refresh_token` and `clientid`
    /// if successful `TDauth` will carry new valid `token`
    /// if refreshupdate is true than `refresh_token` will also be updated
    pub fn new_fromrefresh(refresh: &str, clientid: &str, refreshupdate: bool) -> TDauth {
        let mut newauth = TDauth {
            token: String::new(),
            refresh: refresh.to_owned(),
            clientid: format!("{}{}", clientid, crate::APIKEY),
            redirecturi: None,
        };
        newauth.resolve_token_fromrefresh(refreshupdate);
        newauth
    }
    /// create new `TDauth` with `code`, `redirecturi` and `clientid`
    /// if successful `TDauth` will carry both new `refresh_token` and new valid `token`
    ///
    /// you can use decode=true if you did **NOT** decode it **only useful if you are using the browser to get code from query string**
    ///
    pub fn new_fromcode(code: &str, clientid: &str, redirecturi: &str, codedecode: bool) -> TDauth {
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
    ///
    /// returns full response and updates `TDauth` struct
    ///
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
            .expect(R_ERR)
            .text()
            .expect(R_ERR);

        let responsejson: serde_json::Value = serde_json::from_str(&response).expect(T_ERR);
        self.token = responsejson["access_token"]
            .as_str()
            .expect(T_ERR)
            .to_owned();
        if refreshupdate {
            self.refresh = responsejson["refresh_token"]
                .as_str()
                .expect(T_ERR)
                .to_owned();
        }
        response
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
            .expect(R_ERR)
            .text()
            .expect(R_ERR);

        let responsejson: serde_json::Value =
            serde_json::from_str(&response).expect("Error: No access token retrieved");
        self.token = responsejson["access_token"]
            .as_str()
            .expect(T_ERR)
            .to_owned();
        self.refresh = responsejson["refresh_token"]
            .as_str()
            .expect(T_ERR)
            .to_owned();
        response
    }

    pub fn gettokens(&self) -> (&str, &str) {
        (&self.token, &self.refresh)
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
        let newtdauth = TDauth::new_fromrefresh(&refresh, &clientid, false);
        let (t, r) = newtdauth.gettokens();
        println!("token: {} \nrefresh: {} \n", t, r);
    }
}
