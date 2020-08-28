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
//! # Auth module
//!
//! Auth module can be used separately to renew tokens or to construct a weblink to grab an authroization code.
//! See instructions in module.
//!

static APIWWW: &str = "https://api.tdameritrade.com/v1/";
static APIKEY: &str = "@AMER.OAUTHAP";

mod param;
mod tdaclient;

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
