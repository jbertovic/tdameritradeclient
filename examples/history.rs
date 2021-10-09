use std::env;
use tdameritradeclient::{param, Endpoint, TDAClient};

fn main() {
    env_logger::init();
    let c = TDAClient::new(env::var("TDAUTHTOKEN").unwrap());
    title_print("History:");
    pretty_print(&c.get(
        &Endpoint::History("SPY"),
        &[
            param::History::Period(1),
            param::History::PeriodType("month"),
            param::History::Frequency(1),
            param::History::FrequencyType("daily"),
        ],
    ));
}

fn pretty_print(toprint: &serde_json::Value) {
    println!("{}\n", serde_json::to_string_pretty(toprint).unwrap());
}

fn title_print(heading: &str) {
    println!("{}", heading.to_uppercase());
    println!("{}", "-".repeat(heading.len()));
}
