//! # tdameritradeclient
//!
//! TDAClient is the main struct that lets you build requests to TDAmeritrade's API
//!
//! See [TD Ameritrade API](http://developer.tdameritrade.com) for API documentation
//!
//! Create TDAClient with a valid Access Token - see [TD Ameritrade API](http://developer.tdameritrade.com) for info on creating token
//!
//! Response output can be kept in text which comes out as JSON text or converted to a `serde_json::Value` object
//!
//! # Query parameters through Enum
//!
//! Use the relevant associated Enums in param to add any parameters to the get function request on the TDAClient
//!
//! # Account module
//!
//! Account module contains a `account::SecuritiesAccount` struct to hold all of the balances, positions, and orders of an account.
//! Convenience functions can be added to work with the account.  Still in development.
//!
//! # Auth module
//!
//! Auth module can be used separately to renew tokens or to construct a weblink to grab an authroization code.
//! See instructions in module.
//!
//! # Example
//!
//! For the example to work you will need to set environmental variables for passing a `token`.  
//!
//! ```
//! use std::env;
//! use tdameritradeclient::TDAClient;
//!
//! // Will need to set TDAUTHTOKEN as environmental variable containing a valid token
//!
//! //set token from environment variables
//! let token = env::var("TDAUTHTOKEN").unwrap();
//!
//! // initiate client
//! let c = TDAClient::new(token);
//!
//! // get quotes for 3 symbols and execute
//! let resptxt: String = c.get_quotes("F,INTC,TRP");
//!
//! // output will be text string in json format
//! println!("{:?}", resptxt);
//! ```
//!

static APIWWW: &str = "https://api.tdameritrade.com/v1/";
static APIKEY: &str = "@AMER.OAUTHAP";

mod param;
mod tdaclient;

///
/// Module containing custom struct for getting and holding accoun balances, positions, and orders
///
pub mod account;
///
/// utility module to help with authorization token, refresh token and grant code
///
/// You can use the public functions or the `TDauth` struct.  `TDauth` allows you to store the information
/// for reuse.
///
pub mod auth;
pub use param::{Account, History, Instruments, OptionChain, Order, Transactions};
///
/// Move to front of crate
///
pub use tdaclient::TDAClient;
