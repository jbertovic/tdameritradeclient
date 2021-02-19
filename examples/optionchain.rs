use std::env;
use tdameritradeclient::{OptionChain, TDAClient};

fn main() {
    env_logger::init();

    let c = TDAClient::new(env::var("TDAUTHTOKEN").unwrap());
    // TODO: add symbol in params
    title_print("Option Chain:");
    pretty_print(&c.get_option_chain(&[
        OptionChain::Symbol("SPY"),
        OptionChain::StrikeCount(3),
        OptionChain::ContractType("CALL"),
    ]));
}

fn pretty_print(toprint: &serde_json::Value) {
    println!("{}\n", serde_json::to_string_pretty(toprint).unwrap());
}

fn title_print(heading: &str) {
    println!("{}", heading.to_uppercase());
    println!("{}", "-".repeat(heading.len()));
}
