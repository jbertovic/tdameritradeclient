use std::env;
use tdameritradeclient::client::{TDAClient, Execute};

//cargo run quote "symbols"
//cargo run userprincipals

fn main() {
    // symbols comma separated with no spaces
    let c = initialize_client();
    let command = env::args().skip(1).next().unwrap();
    match command.as_str() {
        "quote" => {    
            let symbols = env::args().skip(2).next().unwrap();
            let resptxt: String = c.getquotes(&symbols).execute();
            println!("{}", resptxt);
        },
        "userprincipals" => {
            let resptxt: String = c.getuserprincipals().execute();
            println!("{}", resptxt);
        },
        _=>{println!("Command Not Recognized");}
    }
}

fn initialize_client() -> TDAClient {
    let consumerkey = env::var("TDCONSUMERKEY").unwrap();
    let token = env::var("TDAUTHTOKEN").unwrap();
    let c = TDAClient::new(consumerkey, token);
    return c;
}
