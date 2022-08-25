use std::env;
use tdameritradeclient::{error::TDAClientError, param, Endpoint, TDAClient};

fn main() -> Result<(), TDAClientError> {
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
