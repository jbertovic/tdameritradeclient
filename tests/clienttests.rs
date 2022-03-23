// Tests should have some sort of mock to retrieve example json
// These are more like examples
// REQUIRES an active TD ameritrade account and valid token

// TODO: tests to add: watchlist endpoints

use tdameritradeclient::model::pricehistory::History;
use tdameritradeclient::model::account::AccountRoot;
use tdameritradeclient::model::userprincipals::UserPrincipals;
use std::env;
use tdameritradeclient::{param, Endpoint, TDAClient};

fn initialize_client() -> TDAClient {
    TDAClient::new(env::var("TDAUTHTOKEN").unwrap())
}

fn initialize_client_accountid() -> (TDAClient, String) {
    let c = initialize_client();
    let user: serde_json::Value = c.get(&Endpoint::UserPrincipals, &[param::Empty]);
    let accountid = user["primaryAccountId"]
        .as_str()
        .expect("Trouble Parsing Primary AccountId")
        .to_owned();
    return (c, accountid);
}

#[test]
fn able_to_retrieve_userprincipals() {
    let resptxt: String = initialize_client().get(&Endpoint::UserPrincipals, &[param::Empty]);
    println!("{:?}", resptxt);
    assert_eq!(resptxt.contains("\"userId\""), true);
}

#[test]
fn able_to_retrieve_userprincipals_into_model() {
    assert!(serde_json::from_value::<UserPrincipals>(initialize_client().get(&Endpoint::UserPrincipals, &[param::Empty])).is_ok());
}

#[test]
fn able_to_retrieve_quotes() {
    let resptxt: String =
        initialize_client().get(&Endpoint::Quotes, &[param::Quotes::Symbol("F,INTC,SPY")]);
    println!("{:?}", resptxt);
    assert_eq!(resptxt.contains("\"assetType\""), true);
}

#[test]
fn able_to_retrieve_tojson() {
    let resptxt: serde_json::Value =
        initialize_client().get(&Endpoint::UserPrincipals, &[param::Empty]);
    println!("{:?}", resptxt);
    assert!(resptxt["userId"].is_string());
}

#[test]
fn able_to_retrieve_history() {
    let resptxt: String = initialize_client().get(
        &Endpoint::History("SPY"),
        &[
            param::History::Period(1),
            param::History::PeriodType("month"),
            param::History::Frequency(1),
            param::History::FrequencyType("daily"),
        ],
    );
    println!("RESULT{:?}", resptxt);
    assert_eq!(resptxt.contains("\"candles\""), true);
}

#[test]
fn able_to_retrieve_pricehistory_into_model() {
    assert!(serde_json::from_value::<History>(initialize_client()
        .get(&Endpoint::History("SPY"), 
            &[
                param::History::Period(1),
                param::History::PeriodType("month"),
                param::History::Frequency(1),
                param::History::FrequencyType("daily"),
            ],
        )).is_ok());
}

#[test]
fn able_to_retrieve_optionchain() {
    let resptxt: String = initialize_client().get(
        &Endpoint::OptionChain,
        &[
            param::OptionChain::Symbol("SPY"),
            param::OptionChain::StrikeCount(3),
            param::OptionChain::ContractType("CALL"),
        ],
    );
    println!("{:?}", resptxt);
    assert_eq!(resptxt.contains("\"SUCCESS\""), true);
}

#[test]
fn able_to_retrieve_all_accounts() {
    let resptxt: String = initialize_client().get(&Endpoint::Accounts, &[param::Empty]);
    println!("{:?}", resptxt);
    assert_eq!(resptxt.contains("\"securitiesAccount\""), true);
}

#[test]
fn able_to_retrieve_one_account() {
    let (c, accountid) = initialize_client_accountid();
    let resptxt: String = c.get(&Endpoint::Account(&accountid), &[param::Empty]);
    println!("{:?}", resptxt);
    assert_eq!(resptxt.contains("\"securitiesAccount\""), true);
}

#[test]
fn able_to_retrieve_account_into_model() {
    let (c, accountid) = initialize_client_accountid();
    assert!(serde_json::from_value::<AccountRoot>(c.get(&Endpoint::Account(&accountid), &[param::Empty])).is_ok());
    assert!(serde_json::from_value::<AccountRoot>(c.get(&Endpoint::Account(&accountid), &[param::Account::Positions])).is_ok());
    assert!(serde_json::from_value::<AccountRoot>(c.get(&Endpoint::Account(&accountid), &[param::Account::Orders])).is_ok());
}


#[test]
fn able_to_retrieve_account_positions() {
    let (c, accountid) = initialize_client_accountid();
    let resptxt: String = c.get(&Endpoint::Account(&accountid), &[param::Account::Positions]);
    println!("{:?}", resptxt);
    assert_eq!(resptxt.contains("\"positions\""), true);
}

#[test]
fn able_to_retrieve_transactions() {
    let (c, accountid) = initialize_client_accountid();
    let resptxt: String = c.get(&Endpoint::Transactions(&accountid), &[param::Empty]);
    println!("{:?}", resptxt);
    assert_eq!(resptxt.contains("\"transactionItem\""), true);
}

#[test]
fn able_to_retrieve_instrument_cusip() {
    let c = initialize_client();
    let resptxt: String = c.get(&Endpoint::Instrument("458140100"), &[param::Empty]);
    println!("{:?}", resptxt);
    assert_eq!(resptxt.contains("\"cusip\""), true);
}

#[test]
fn able_to_retrieve_instrument_search() {
    let c = initialize_client();
    let resptxt: String = c.get(
        &Endpoint::Instruments,
        &[
            param::Instruments::Symbol("INTC"),
            param::Instruments::SearchType("fundamental"),
        ],
    );
    println!("{:?}", resptxt);
    assert_eq!(resptxt.contains("\"cusip\""), true);
}
