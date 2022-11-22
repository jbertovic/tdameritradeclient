use serde::{Deserialize, Deserializer};
use serde_json::Value;

/// type to respresent responses from /accounts/ endpoint
pub mod account;
/// type to respresent responses from /instruments
pub mod instrument;
/// type to represent optionchains from /marketdata/chains
pub mod optionchain;
/// type to represent price history from /marketdata/SYM/pricehistory
pub mod pricehistory;
/// type to represent quotes from /marketdata/quotes?SYM1,SYM2,etc
pub mod quote;
/// type to represent token authorization response from /oauth2/token endpoint
pub mod token;
/// type to respresent responses from /userprincipals/ endpoint
pub mod userprincipals;

fn ok_or_default<'a, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'a> + Default,
    D: Deserializer<'a>,
{
    let v: Value = Deserialize::deserialize(deserializer)?;
    Ok(T::deserialize(v).unwrap_or_default())
}
