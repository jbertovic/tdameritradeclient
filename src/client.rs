static APIWWW: &str = "https://api.tdameritrade.com/v1/";
use attohttpc::{RequestBuilder, Session};
//use serde_json::Value;
/*
* two options for output -> text which in this case is JSON from TDA API, or -> convert to Hashmap
* this way who ever is using the library can pick what they want
* able to pick which keys you want returned in hash map or maybe a way of reducing it to a paticular native format within the library
*/

#[derive(Debug)]
pub struct TDAClient {
    consumerkey: String,
    authtoken: String,
    pub client: Session,
}

#[allow(dead_code)]
impl TDAClient {
    pub fn new(consumerkey: String, token: String) -> TDAClient {
        let mut client = Session::new();
        client.header("AUTHORIZATION", format!("Bearer {}", &token));

        TDAClient {
            consumerkey: consumerkey,
            authtoken: token,
            client: client,
        }
    }

    pub fn getuserprincipals(&self) -> RequestBuilder {
        self.client.get(format!("{}userprincipals", APIWWW))
    }

    pub fn getquotes(&self, quotequery: &str) -> RequestBuilder {
        self.client
            .get(format!("{}marketdata/quotes", APIWWW))
            .param("symbol", quotequery)
    }
}

// TODO: Error Checking
impl Executor for RequestBuilder {
    fn exec_to_text(self) -> String {
        self.send().unwrap().text().unwrap()
    }

    fn exec_to_jsonobject(self) -> serde_json::Value {
        serde_json::from_str(self.send().unwrap().text().unwrap().as_str()).unwrap()
    }
}
pub trait Executor {
    fn exec_to_jsonobject(self) -> serde_json::Value;
    fn exec_to_text(self) -> String; 
}

#[cfg(test)]
mod tests_tdaclient {
    use super::*;
    use std::env;

    fn initialize_client() -> TDAClient {
        let consumerkey = env::var("TDCONSUMERKEY").unwrap();
        let token = env::var("TDAUTHTOKEN").unwrap();
        let c = TDAClient::new(consumerkey, token);
        return c;
    }

    #[test]
    fn able_to_retrieve_user_data() {
        let c = initialize_client();
        let resptxt = c.getuserprincipals().exec_to_text();
        println!("{:?}", resptxt);
        assert_eq!(resptxt.starts_with("{\n  \"authToken\""), true);
    }
    #[test]
    fn able_to_retrieve_quotes() {
        let c = initialize_client();
        let resptxt = c.getquotes("F,INTC,TRP").exec_to_text();
        println!("{:?}", resptxt);
        assert_eq!(resptxt.contains("\"assetType\""), true);
    }
    #[test]
    fn able_to_retrieve_tojson() {
        let c = initialize_client();
        let resptxt = c.getuserprincipals().exec_to_jsonobject();
        println!("{:?}", resptxt);
        assert!(resptxt["userId"].is_string());
    }

}
