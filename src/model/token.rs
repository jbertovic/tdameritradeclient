use serde::Deserialize;
///
/// Holds Type Authorization response
///
#[derive(Default, Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub scope: String,
    pub refresh_token_expires_in: u64,
}

#[derive(Default, Debug, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}
