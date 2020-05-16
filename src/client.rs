static APIWWW: &str = "https://api.tdameritrade.com/v1/";
use attohttpc::{RequestBuilder, Session};
//use serde_json::Value;
/*
* two options for output -> text which in this case is JSON from TDA API, or -> convert to serde_json::Value;
* this way who ever is using the library can pick what they want
* able to pick which keys you want returned in hash map or maybe a way of reducing it to a paticular native format within the library
*
* TODO: quote history
* TODO: option chain
* TODO: return text or json value modified by search strings
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

    pub fn gethistory(&self, symbol: &str) -> RequestBuilder {
        self.client
        .get(format!("{}marketdata/{}/pricehistory", APIWWW, symbol))
    }

    pub fn getoptionchain(&self) -> RequestBuilder {
        unimplemented!();
    }
}

enum History<'a> {
    PeriodType(&'a str),
    Period(u8),
    FrequencyType(&'a str),
    Frequency(u8),
    StartDate(u64),
    EndDate(u64),
    NeedExendedHoursData(bool),
}

impl<'a> History<'a> {
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

    fn pair(self) -> [(&'static str, String);1] {
        [self.into();1]
    }
}

// TODO: Check if Error is returned with bad token or expired or invalid
pub trait Execute<T> {
    fn execute(self) -> T;
}

impl Execute<String> for RequestBuilder {
    fn execute(self) -> String {
        self
            .send().expect("Trouble Retrieving Response: ERROR")
            .text().expect("Response did not return JSON: ERROR")
    }
}

impl Execute<serde_json::Value> for RequestBuilder {
    fn execute(self) -> serde_json::Value {
        serde_json::from_str(self
            .send().expect("Trouble Retrieving Response: ERROR")
            .text().expect("Response did not return JSON: ERROR")
            .as_str()).expect("SERDE: Trouble parsing json text: ERROR")
    }
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
        let resptxt: String = c.getuserprincipals().execute();
        println!("{:?}", resptxt);
        assert_eq!(resptxt.starts_with("{\n  \"authToken\""), true);
    }
    #[test]
    fn able_to_retrieve_quotes() {
        let c = initialize_client();
        let resptxt: String = c.getquotes("F,INTC,TRP").execute();
        println!("{:?}", resptxt);
        assert_eq!(resptxt.contains("\"assetType\""), true);
    }
    #[test]
    fn able_to_retrieve_tojson() {
        let c = initialize_client();
        let resptxt: serde_json::Value = c.getuserprincipals().execute();
        println!("{:?}", resptxt);
        assert!(resptxt["userId"].is_string());
    }

    #[test]
    fn able_to_retrieve_history() {
        let c = initialize_client();
        let resptxt: String = c
            .gethistory("TRP")
            .params(&History::Period(1).pair())
            .params(&History::PeriodType("month").pair())
            .params(&History::Frequency(1).pair())
            .params(&History::FrequencyType("daily").pair())
            .execute();
        println!("{:?}", resptxt);
        assert_eq!(resptxt.contains("\"candles\""), true);
    }
}
