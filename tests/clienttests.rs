// Tests should have some sort of mock to retrieve example json
// These are more like examples
// REQUIRES an active TD ameritrade account and valid token

use tdameritradeclient::{TDAClient, History, Account, OptionChain, Instruments};
use std::env;

fn initialize_client() -> TDAClient {
    TDAClient::new(env::var("TDAUTHTOKEN").unwrap())
}

fn initialize_client_accountid() -> (TDAClient, String) {
    let c = initialize_client();
    let user: serde_json::Value = c.getuserprincipals();
    let accountid = user["primaryAccountId"].as_str().expect("Trouble Parsing Primary AccountId").to_owned();
    return (c, accountid);
}

#[test]
fn able_to_retrieve_user_data() {
    let resptxt: String = initialize_client().getuserprincipals();
    println!("{:?}", resptxt);
    assert_eq!(resptxt.starts_with("{\n  \"authToken\""), true);
}

#[test]
fn able_to_retrieve_quotes() {
    let resptxt: String = initialize_client().getquotes("F,INTC,SPY");
    println!("{:?}", resptxt);
    assert_eq!(resptxt.contains("\"assetType\""), true);
}

#[test]
fn able_to_retrieve_tojson() {
    let resptxt: serde_json::Value = initialize_client().getuserprincipals();
    println!("{:?}", resptxt);
    assert!(resptxt["userId"].is_string());
}

#[test]
fn able_to_retrieve_history() {
    let resptxt: String = initialize_client().gethistory(
        "SPY",
        &[
            History::Period(1),
            History::PeriodType("month"),
            History::Frequency(1),
            History::FrequencyType("daily"),
        ],
    );
    println!("RESULT{:?}", resptxt);
    assert_eq!(resptxt.contains("\"candles\""), true);
}

#[test]
fn able_to_retrieve_optionchain() {
    let resptxt: String = initialize_client().getoptionchain(
        &[
            OptionChain::Symbol("SPY"),
            OptionChain::StrikeCount(3),
            OptionChain::ContractType("CALL"),
        ],
    );
    println!("{:?}", resptxt);
    assert_eq!(resptxt.contains("\"SUCCESS\""), true);
}

#[test]
fn able_to_retrieve_all_accounts() {
    let resptxt: String = initialize_client().getaccounts();
    println!("{:?}", resptxt);
    assert_eq!(resptxt.contains("\"securitiesAccount\""), true);
}

#[test]
fn able_to_retrieve_one_account() {
    let (c, accountid) = initialize_client_accountid();
    let resptxt: String = c.getaccount(&accountid, &[]);
    println!("{:?}", resptxt);
    assert_eq!(resptxt.contains("\"securitiesAccount\""), true);
}

#[test]
fn able_to_retrieve_account_positions() {
    let (c, accountid) = initialize_client_accountid();
    let resptxt: String = c.getaccount(
        &accountid,
        &[Account::Positions],
    );
    println!("{:?}", resptxt);
    assert_eq!(resptxt.contains("\"positions\""), true);
}

#[test]
fn able_to_retrieve_transactions() {
    let (c, accountid) = initialize_client_accountid();
    let resptxt: String = c.gettransactions(
        &accountid,
        &[],
    );
    println!("{:?}", resptxt);
    assert_eq!(resptxt.contains("\"transactionItem\""), true);
}

#[test]
fn able_to_retrieve_instrument_cusip() {
    let c = initialize_client();
    let resptxt: String = c.getinstrument("458140100");
    println!("{:?}", resptxt);
    assert_eq!(resptxt.contains("\"cusip\""), true);
}

#[test]
fn able_to_retrieve_instrument_search() {
    let c = initialize_client();
    let resptxt: String = c.getinstruments(
        &[
            Instruments::Symbol("INTC"),
            Instruments::SearchType("fundamental")],
    );
    println!("{:?}", resptxt);
    assert_eq!(resptxt.contains("\"cusip\""), true);
}