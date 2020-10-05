//! Handles management OAuth token retrieval, caching, and refreshing.
//! # Examples
//! ```
//! use std::error::Error;
//! use auth0_management::token::TokenManager;
//! use reqwest::Client;
//!
//! async fn get_token() -> Result<String, Box<dyn Error>> {
//!   let mut token_manager = TokenManager::new(
//!     Client::new(),
//!     "MY_DOMAIN.eu.auth0.com",
//!     "https://MY_AUDIENCE",
//!     "MY_CLIENT_ID",
//!     "MY_CLIENT_SECRET",
//!   );
//!
//!   // Encoded JWT token that can be directly sent to Auth0 API via `Authorization` header.
//!   Ok(token_manager.get_token().await?)
//! }
//! ```
use std::error::Error;
use std::time::{SystemTime, SystemTimeError};

use reqwest::Client;
use serde::export::Formatter;
use serde::{Deserialize, Serialize};

/// Auth0 OAuth token.
#[derive(Deserialize)]
pub struct Token {
  // scope: String,
  // token_type: String,
  /// Token expiration in seconds.
  pub expires_in: u64,
  /// Encoded JWT access token.
  pub access_token: String,
}

#[derive(Debug)]
pub enum TokenError {
  ReqwestErr(reqwest::Error),
  SystemTimeErr(SystemTimeError),
}

#[derive(Serialize, Clone, Debug)]
struct TokenOpts {
  audience: String,
  grant_type: String,
  client_id: String,
  client_secret: String,
}

/// Provides oauth token retrieval and expiration checks.
pub struct TokenManager {
  client: Client,
  pub(crate) domain: String,

  pub token: Option<Token>,
  token_opts: TokenOpts,
  pub token_last: SystemTime,
}

impl TokenManager {
  /// Gets builder for [TokenManager].
  pub fn new(
    client: Client,
    domain: &str,
    audience: &str,
    client_id: &str,
    client_secret: &str,
  ) -> Self {
    Self {
      client,
      domain: domain.to_owned(),
      token: None,
      token_opts: TokenOpts {
        audience: audience.to_owned(),
        grant_type: "client_credentials".to_owned(),
        client_id: client_id.to_owned(),
        client_secret: client_secret.to_owned(),
      },
      token_last: SystemTime::UNIX_EPOCH,
    }
  }

  /// Gets valid encoded JWT token.
  pub async fn get_token(&mut self) -> Result<String, TokenError> {
    match &self.token {
      None => Ok(self.fetch_token().await.map_err(TokenError::ReqwestErr)?),
      Some(token) => {
        let elapsed = SystemTime::now()
          .duration_since(self.token_last)
          .map_err(TokenError::SystemTimeErr)?;

        if elapsed.as_secs() > token.expires_in {
          Ok(self.fetch_token().await.map_err(TokenError::ReqwestErr)?)
        } else {
          Ok(self.token.as_ref().unwrap().access_token.to_owned())
        }
      }
    }
  }

  /// Gets new encoded JWT token from auth0.
  async fn fetch_token(&mut self) -> Result<String, reqwest::Error> {
    let token = self
      .client
      .post(&format!("https://{}/oauth/token", self.domain))
      .form(&self.token_opts)
      .send()
      .await?
      .json()
      .await?;

    self.token = Some(token);
    self.token_last = SystemTime::now();

    Ok(self.token.as_ref().unwrap().access_token.to_owned())
  }
}

impl std::fmt::Display for TokenError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl Error for TokenError {}
