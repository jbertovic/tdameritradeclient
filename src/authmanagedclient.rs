use crate::auth::TDauth;
use crate::TDAClient;
use crate::REFRESHTIMEBUFFER;
use crate::TOKENTIMEBUFFER;
use log::trace;

/// Wrapper around `TDAClient` that handles managing the authorization token
///
/// A valid refresh token and client id needs to be supplied
/// or a TDAuth struct
///
#[derive(Debug, Clone)]
pub struct TDAClientAuth {
    client: TDAClient,
    auth: TDauth,
}

impl TDAClientAuth {
    /// create a new managed client with refresh token and client id that will check and refresh tokens
    /// as needed before every use.
    pub fn new(refresh_token: String, client_id: String) -> Self {
        trace!("New Client (Auth Managed) initialized - from refresh token");
        let auth = TDauth::new_from_refresh(&refresh_token, &client_id, true);
        TDAClientAuth {
            client: TDAClient::new(auth.get_auth_token().to_owned()),
            auth,
        }
    }

    /// create a new managed client from a TDauth configured struct that will check and refresh tokens
    /// as needed before every use.
    pub fn from_tdauth(auth: TDauth) -> Self {
        trace!("New Client (Auth Managed) initialized - from TDauth struct");
        TDAClientAuth {
            client: TDAClient::new(auth.get_auth_token().to_owned()),
            auth,
        }
    }

    /// retrieve client with updated token to use
    /// return None if no token exists
    pub fn client(&mut self) -> Option<&TDAClient> {
        // check validity of token
        if !self.check_token_validity() && !self.auth.get_auth_token().is_empty() {
            // update client with new token if token exists otherwise leave existing client
            self.client = TDAClient::new(self.auth.get_auth_token().to_owned());
        }
        if self.auth.get_auth_token().is_empty() {
            None
        } else {
            Some(&self.client)
        }
    }

    /// get current authorization token for TD API
    pub fn active_token(&mut self) -> Option<&str> {
        self.check_token_validity();
        if self.auth.get_auth_token().is_empty() {
            None
        } else {
            Some(self.auth.get_auth_token())
        }
    }

    fn check_token_validity(&mut self) -> bool {
        // check validity of token
        if !self.auth.is_token_valid(TOKENTIMEBUFFER) || self.auth.get_auth_token().is_empty() {
            // if token needs updating check if refresh needs to be updated too
            let refresh_update = !self.auth.is_refresh_valid(REFRESHTIMEBUFFER);
            self.auth.resolve_token_from_refresh(refresh_update);
            false
        } else {
            true
        }
    }

    /// check that an active refresh token exists so client tokens can be updated as needed
    ///
    /// if this is false then `TDauth` will have to be resolved by getting a refresh token
    /// probably using `tdauth_code_grant`
    pub fn refresh_auth_active(&self) -> bool {
        self.auth.is_refresh_valid(REFRESHTIMEBUFFER)
    }

    /// get authorization tokens for TD API
    pub fn get_auth(&self) -> &TDauth {
        &self.auth
    }
}

#[cfg(test)]
mod managed_client_tests {

    use super::TDAClientAuth;
    use crate::{param, Endpoint};
    use std::env;

    #[test]
    #[ignore]
    fn check_managed_client() {
        let refresh = env::var("TDREFRESHTOKEN").unwrap();
        let clientid = env::var("TDCLIENTKEY").unwrap();
        let mut managed_client = TDAClientAuth::new(refresh, clientid);

        let resptxt: String = managed_client
            .client()
            .unwrap()
            .get(&Endpoint::Quotes, &[param::Quotes::Symbol("F,INTC,SPY")])
            .unwrap();
        assert_eq!(resptxt.contains("\"assetType\""), true);

        let (t1, r1, t2, r2): (String, String, String, String);
        {
            let (token1, refresh1) = managed_client.auth.get_tokens();
            t1 = token1.to_owned();
            r1 = refresh1.to_owned();
        }
        // set tokens to expire
        managed_client.auth.reset_expire();

        // check that both tokens are valid after another request
        let resptxt: String = managed_client
            .client()
            .unwrap()
            .get(&Endpoint::Quotes, &[param::Quotes::Symbol("F,INTC,SPY")])
            .unwrap();
        assert_eq!(resptxt.contains("\"assetType\""), true);

        {
            let (token2, refresh2) = managed_client.auth.get_tokens();
            t2 = token2.to_owned();
            r2 = refresh2.to_owned();
        }

        assert_ne!(t1, t2);
        assert_ne!(r1, r2);
        assert!(&managed_client.auth.is_token_valid(0));
        assert!(&managed_client.auth.is_refresh_valid(0));
    }
}
