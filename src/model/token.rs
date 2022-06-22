use serde::Deserialize;
///
/// Holds Type Authorization response
///
#[derive(Default, Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    #[serde(default)]
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: u64,
    #[serde(default)]
    pub scope: String,
    #[serde(default)]
    pub refresh_token_expires_in: u64,
}

#[derive(Default, Debug, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}
