use crate::auth::TDauth;
use crate::TDAClient;
use crate::REFRESHTIMEBUFFER;
use crate::TOKENTIMEBUFFER;
use log::info;

/// Wrapper around `TDAClient` that handles managing the authorization token
///
/// A valid refresh token and client id needs to be supplied
///
/// Initiate with a valid refresh token and client id
///
#[derive(Debug, Clone)]
pub struct TDAClientAuth {
    client: TDAClient,
    auth: TDauth,
}

impl TDAClientAuth {
    /// create a new managed client that will check and refresh tokens as needed before
    /// every use.
    pub fn new(refresh_token: String, client_id: String) -> Self {
        info!("New Client (Auth Managed) initialized - from refresh token");
        let auth = TDauth::new_from_refresh(&refresh_token, &client_id, true);
        TDAClientAuth {
            client: TDAClient::new(auth.get_auth_token().to_owned()),
            auth,
        }
    }

    /// retrieve client with updated token to use
    pub fn client(&mut self) -> &TDAClient {
        // check validity of token
        if !self.auth.is_token_valid(TOKENTIMEBUFFER) {
            // if token needs updating check if refresh needs to be updated too
            let refresh_update = !self.auth.is_refresh_valid(REFRESHTIMEBUFFER);
            self.auth.resolve_token_from_refresh(refresh_update);

            // update client with new token
            self.client = TDAClient::new(self.auth.get_auth_token().to_owned());
        }
        &self.client
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
            .get(&Endpoint::Quotes, &[param::Quotes::Symbol("F,INTC,SPY")]);
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
            .get(&Endpoint::Quotes, &[param::Quotes::Symbol("F,INTC,SPY")]);
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
