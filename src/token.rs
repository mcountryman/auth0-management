//! Handles management OAuth token retrieval, caching, and refreshing.
//! # Examples
//! ```
//! use std::error::Error;
//! use auth0_management::token::TokenManager;
//!
//! async fn get_token() -> Result<String, Box<dyn Error>> {
//!   let mut token_manager = TokenManager::new()
//!     .domain("MY_DOMAIN.eu.auth0.com")
//!     .audience("https:://MY_AUDIENCE")
//!     .client_id("MY_CLIENT_ID")
//!     .client_secret("MY_CLIENT_SECRET")
//!     .build()?;
//!
//!   // Encoded JWT token that can be directly sent to Auth0 API via `Authorization` header.
//!   Ok(token_manager.get_token().await?)
//! }
//! ```
use std::error::Error;
use std::time::SystemTime;

use reqwest::Client;
use serde::export::fmt::Display;
use serde::export::Formatter;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialOrd, PartialEq)]
pub enum TokenManagerError {
  MissingDomain,
  MissingAudience,
  MissingClientID,
  MissingClientSecret,
}

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

#[derive(Serialize, Clone, Debug)]
struct TokenOpts {
  audience: String,
  grant_type: String,
  client_id: String,
  client_secret: String,
}

/// Builder for [TokenManager].
pub struct TokenManagerBuilder {
  pub(crate) domain: Option<String>,
  audience: Option<String>,
  client_id: Option<String>,
  client_secret: Option<String>,
}

impl TokenManagerBuilder {
  pub fn new() -> Self {
    TokenManagerBuilder {
      domain: None,
      audience: None,
      client_id: None,
      client_secret: None,
    }
  }

  /// Auth0 tenant domain.
  pub fn domain(mut self, domain: &str) -> Self {
    self.domain = Some(domain.to_owned());
    self
  }

  /// Unique identifier for the API.  Can be found on the `Settings` tab of the API.
  pub fn audience(mut self, audience: &str) -> Self {
    self.audience = Some(audience.to_owned());
    self
  }

  /// Client ID of application.  Can be found on the `Settings` tab of the Machine-to-Machine
  /// Application.
  pub fn client_id(mut self, client_id: &str) -> Self {
    self.client_id = Some(client_id.to_owned());
    self
  }

  /// Client secret of application.  Can be found on the `Settings` tab of the Machine-to-Machine
  /// Application.
  pub fn client_secret(mut self, client_secret: &str) -> Self {
    self.client_secret = Some(client_secret.to_owned());
    self
  }

  /// Create token manager.
  pub fn build(self) -> Result<TokenManager, TokenManagerError> {
    Ok(TokenManager {
      domain: self.domain.ok_or(TokenManagerError::MissingDomain)?,

      client: Default::default(),

      token: None,
      token_opts: TokenOpts {
        audience: self.audience.ok_or(TokenManagerError::MissingAudience)?,
        grant_type: "client_credentials".to_string(),
        client_id: self.client_id.ok_or(TokenManagerError::MissingClientID)?,
        client_secret: self
          .client_secret
          .ok_or(TokenManagerError::MissingClientSecret)?,
      },
      token_last: SystemTime::UNIX_EPOCH,
    })
  }
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
  pub fn new() -> TokenManagerBuilder {
    TokenManagerBuilder::new()
  }

  /// Gets valid encoded JWT token.
  pub async fn get_token(&mut self) -> Result<String, Box<dyn Error + Send + Sync>> {
    match &self.token {
      None => Ok(self.fetch_token().await?),
      Some(token) => {
        let elapsed = SystemTime::now().duration_since(self.token_last)?;
        if elapsed.as_secs() > token.expires_in {
          Ok(self.fetch_token().await?)
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

impl Display for TokenManagerError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl Error for TokenManagerError {}
