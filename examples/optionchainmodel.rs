use std::collections::HashMap;
use std::env;
use tdameritradeclient::model::optionchain::{OptionChain, OptionQuote};
use tdameritradeclient::{param, Endpoint, TDAClient};

fn main() {
    env_logger::init();

    // grab authorization token from an environmental variable
    let c = TDAClient::new(env::var("TDAUTHTOKEN").unwrap());

    // get userprincipals endpoint and parse into userprincipals type
    title_print("Option Chain Quote for SPY:");

    let optionchain: OptionChain =
        serde_json::from_value(
            c.get(
                &Endpoint::OptionChain,
                &[
                    param::OptionChain::Symbol("SPY"),
                    param::OptionChain::StrikeCount(1),
                ],
            )
        ).unwrap();

    println!("Status: {}", &optionchain.status);
    println!("Number of Contracts: {}\n", &optionchain.number_of_contracts);

    title_print("PUTS");
    print_exp_date_map(&optionchain.put_exp_date_map);

    title_print("CALLS");
    print_exp_date_map(&optionchain.call_exp_date_map);

}

fn title_print(heading: &str) {
    println!("{}", heading.to_uppercase());
    println!("{}", "-".repeat(heading.len()));
}

fn print_exp_date_map(exp_date_map: &HashMap<String, HashMap<String, Vec<OptionQuote>>>) {
    for optionmap in exp_date_map.values() {
        for optionquote in optionmap.values() {
            println!("{:?}\n", optionquote);
        }
    }
}