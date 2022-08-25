use std::env;
use tdameritradeclient::{error::TDAClientError, param, Endpoint, TDAClient};

fn main() -> Result<(), TDAClientError> {
    env_logger::init();

    let c = TDAClient::new(env::var("TDAUTHTOKEN").unwrap());
    // TODO: add symbol in params
    title_print("Option Chain:");
    pretty_print(&c.get(
        &Endpoint::OptionChain,
        &[
            param::OptionChain::Symbol("SPY"),
            param::OptionChain::StrikeCount(3),
            param::OptionChain::ContractType("CALL"),
        ],
    )?);

    Ok(())
}

fn pretty_print(toprint: &serde_json::Value) {
    println!("{}\n", serde_json::to_string_pretty(toprint).unwrap());
}

fn title_print(heading: &str) {
    println!("{}", heading.to_uppercase());
    println!("{}", "-".repeat(heading.len()));
}
