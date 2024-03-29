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
//! # Client request functions
//!
//! Use the relevant API endpoint with `request::Endpoint` and query parameters `param`.  When no query parameters are necessary use `param::Empty`.
//!
//! See `TDAClient` for each of the request functions: `get`, `post`, `put`, `patch`, and `delete`.
//!
//! # Model module
//!
//! Model module contains response types that can be parsed from the JSON responses.  This is a work in progress and still in development.
//!
//! I tried using TD Ameritrade Schema's that were located on their developer site, however, they don't always match or there is additional data
//!  available.  Therefore, I'm creating these response type's manually - experimenting a bit.  Would be much more useful if there was a way to
//!  code generate the types that match each response.  Its your option if you want to use them, or stick with json or define your own response types.
//!
//! # Auth module
//!
//! Auth module can be used separately to renew tokens or to construct a weblink to grab an authroization code.
//! See instructions in module.  Also works with the managed client `TDAClientAuth`.
//!
//! # Basic Client
//!
//! Using `tdameritradeclient::TDAClient` will require a valid token.  Token management will need to be managed
//! by the user with tools in the `auth` module.
//!
//! # Managed Client
//!
//! Using `tdameritradeclient::TDAClientAuth` will require a valid refresh token and client id.  Token management
//! can be managed by this client.  `TDAClientAuth` is a wrapper around `TDAClient` but includes authorization
//! information.  You can access an updated client and make all the same requests as `TDAClient`.
//!
//! # Example
//!
//! For the example to work you will need to set environmental variables for passing a `token`.  
//!
//! ```
//! use std::env;
//! use tdameritradeclient::{TDAClient, Endpoint, param};
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
//! let resptxt: String = c.get(&Endpoint::Quotes, &[param::Quotes::Symbol("F,INTC,TRP")]).unwrap();
//!
//! // output will be text string in json format
//! println!("{:?}", resptxt);
//! ```
//!

static APIWWW: &str = "https://api.tdameritrade.com/v1/";
static APIKEY: &str = "@AMER.OAUTHAP";
const TOKENTIMEBUFFER: u64 = 25 * 60; // 25 Minutes instead of 30 min
const REFRESHTIMEBUFFER: u64 = 60 * 24 * 60; // 60 days instead of 90 days
type Result<T> = std::result::Result<T, error::TDAClientError>;

///
/// utility module to help with authorization token, refresh token and grant code
///
/// You can use the public functions or the `TDauth` struct.  `TDauth` allows you to store the information
/// for reuse.
///
pub mod auth;
/// holds all the available query parameters used with the endpoints
pub mod param;
/// holds all the relevant API endpoints
pub mod request;
/// Move to front of crate
pub use request::Endpoint;

mod tdaclient;
/// client that manages session and interaction with TDAmeritrade API
pub use tdaclient::TDAClient;

mod authmanagedclient;
/// client that manages authorization, sessions and interaction with TDAmeritrade API
pub use authmanagedclient::TDAClientAuth;

/// models that define types to parse json response or value responses from API
///
/// Used: `https://transform.tools/json-to-rust-serde` to help with struct generation
///
pub mod model;

/// Rolled up Errors for TDAClient
pub mod error;
