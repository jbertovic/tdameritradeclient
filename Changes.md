# v0.4.0
 - Major change in approach of making requests.  Requests are now made from an enum of endpoints instead of individual unique functions on client for each endpoint.
 - added generic requests on client; `get`, `post`, `put`, `patch` and `delete`
 - removed endpoint funtions from `TDClient` which will prevent backward compatibility to version 0.3
 - caputured endpoints in `request` module
 - updated documentation, examples, and tests with new approach
 - added model types for `SecuritiesAccount` and `UserPrincipals` response

# v0.3.1
- added `derive(Default)` on `TDClient`

# v0.3.0
- add log crate and inserted a few log outputs for ERROR and INFO


# v0.2.2
- added endpoint `/instruments/` and `/instruments/{cusip}`
- added endpoint `/accounts/transactions/` and `/accounts/transactions/{transactionid}`
- added additional option for timeout on `attohttpc::session`


# v0.2.1
- added expect() on any unwraps for error panic messages
- able to refresh the refresh token - added bool: new_fromrefresh(refresh: &str, clientid: &str, refreshupdate: bool)
- getter method to retrieve tokens: gettokens(&self) -> (&str, &str)
- convenience method to get updated refresh token: getrefresh_fromrefresh(refresh: &str, clientid: &str)


# v0.2.0
- added `/oauth2/token` endpoint for API through new `auth` module
- to manage tokens added `TDauth` struct along with functions to use `/oauth2/token` endpoint
- added `[ignore]` unit tests for `TDauth` to validate that fetch token works
- `TDClient` can now be created with `refresh_token` or `code` which will fetch a valid `token`
- added `[ignore]` unit tests for `TDClient` to make sure alternate constructors work
- updated documentation throughout including `Readme.md`


# v0.1.2

- fixed `/src/param.rs` to correct bug for `OptionChain` Enum
- updated documentation OptionChain parameters


# v0.1.1

- added disclaimer
- fixed `/src/param.rs` to correct bug for `Order` Enum
- small corrections to `Readme.md`


# v0.1.0

Created initial version 

Completed TODO's
- [X] grab account data (endpoint below /accounts)
- [X] How do I use the enum better to correspond to the proper endpoint?
- [X] able to view saved and current orders with filter (endpoint below /saveorders)
- [X] modified so param are forced to enum and removed builder from pub
- [*] **(NOT USED)**able to create and delete saved order  (endpoint below with POST and DELETE /saveorders)
- [X] create example to pull history, optionchain, and quote
- [X] able to specify type of orders to retrieve - add `Order` Enum
- [X] able to create, change and delete order (endpoint with POST, PUT and DELETE /orders)
- [X] create example to create order
