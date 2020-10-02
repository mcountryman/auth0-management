//! Handles management OAuth token retrieval, caching, and refreshing.
//! # Examples
//! ```
//! use std::error::Error;
//! use auth0_management::token::ClientTokenManager;
//!
//! async fn get_token() -> Result<String, Box<dyn Error>> {
//!   let mut token_manager = ClientTokenManager::build()
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
pub enum ClientTokenManagerBuilderError {
  MissingDomain,
  MissingAudience,
  MissingClientID,
  MissingClientSecret,
}

/// Auth0 OAuth token.
#[derive(Deserialize)]
pub struct ClientToken {
  // scope: String,
  // token_type: String,
  /// Token expiration in seconds.
  expires_in: u64,
  /// Encoded JWT access token.
  access_token: String,
}

#[derive(Serialize, Clone, Debug)]
struct ClientTokenOpts {
  audience: String,
  grant_type: String,
  client_id: String,
  client_secret: String,
}

/// Provides oauth token retrieval and expiration checks.
pub struct ClientTokenManager {
  client: Client,
  domain: String,

  token: Option<ClientToken>,
  token_opts: ClientTokenOpts,
  token_last: SystemTime,
}

/// Builder for [ClientTokenManager].
pub struct ClientTokenManagerBuilder {
  domain: Option<String>,
  audience: Option<String>,
  client_id: Option<String>,
  client_secret: Option<String>,
}

impl ClientTokenManagerBuilder {
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
  pub fn build(self) -> Result<ClientTokenManager, ClientTokenManagerBuilderError> {
    Ok(ClientTokenManager {
      domain: self
        .domain
        .ok_or(ClientTokenManagerBuilderError::MissingDomain)?,

      client: Default::default(),

      token: None,
      token_opts: ClientTokenOpts {
        audience: self
          .audience
          .ok_or(ClientTokenManagerBuilderError::MissingAudience)?,
        grant_type: "client_credentials".to_string(),
        client_id: self
          .client_id
          .ok_or(ClientTokenManagerBuilderError::MissingClientID)?,
        client_secret: self
          .client_secret
          .ok_or(ClientTokenManagerBuilderError::MissingClientSecret)?,
      },
      token_last: SystemTime::UNIX_EPOCH,
    })
  }
}

impl ClientTokenManager {
  /// Gets builder for [ClientTokenManager].
  pub fn build() -> ClientTokenManagerBuilder {
    ClientTokenManagerBuilder {
      domain: None,
      audience: None,
      client_id: None,
      client_secret: None,
    }
  }

  /// Gets valid encoded JWT token.
  pub async fn get_token(&mut self) -> Result<String, Box<dyn Error>> {
    match &self.token {
      None => Ok(self.fetch_token().await?),
      Some(token) => {
        if SystemTime::now().duration_since(self.token_last)?.as_secs() > token.expires_in
        {
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

impl Display for ClientTokenManagerBuilderError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl Error for ClientTokenManagerBuilderError {}

#[cfg(test)]
mod tests {
  use crate::token::{ClientToken, ClientTokenManager, ClientTokenManagerBuilderError};
  use std::env::var;
  use std::time::SystemTime;
  use tokio::time::Duration;

  /// Tests ClientTokenManagerBuilder and all the errors that can occur when parameters are not
  /// supplied.
  #[test]
  fn test_build_manager() {
    get_working_client();

    assert!(ClientTokenManager::build().build().is_err());

    assert_eq!(
      ClientTokenManager::build()
        .domain(&var("AUTH0_DOMAIN").unwrap())
        .audience(&var("AUTH0_AUDIENCE").unwrap())
        .client_id(&var("AUTH0_CLIENT_ID").unwrap())
        .build()
        .err()
        .unwrap(),
      ClientTokenManagerBuilderError::MissingClientSecret
    );

    assert_eq!(
      ClientTokenManager::build()
        .domain(&var("AUTH0_DOMAIN").unwrap())
        .audience(&var("AUTH0_AUDIENCE").unwrap())
        .build()
        .err()
        .unwrap(),
      ClientTokenManagerBuilderError::MissingClientID
    );

    assert_eq!(
      ClientTokenManager::build()
        .domain(&var("AUTH0_DOMAIN").unwrap())
        .build()
        .err()
        .unwrap(),
      ClientTokenManagerBuilderError::MissingAudience
    );

    assert_eq!(
      ClientTokenManager::build().build().err().unwrap(),
      ClientTokenManagerBuilderError::MissingDomain
    );
  }

  /// Basic fetch token test.
  #[tokio::test]
  async fn test_get_token() {
    let mut manager = get_working_client();

    // Fetches our first token.
    let token_a = manager.get_token().await.unwrap();
    // Fetch the second token. (should be cached)
    let token_b = manager.get_token().await.unwrap();

    // Ensure our token didn't expire to validate this came from Auth0.
    assert_eq!(token_a, token_b);
  }

  /// Hacky way of testing token expiration.
  #[tokio::test]
  async fn test_update_token() {
    let mut manager = get_working_client();

    // Mock "old" token with 0s expiration date.
    manager.token = Some(ClientToken {
      expires_in: 0,
      access_token: "test".to_owned(),
    });

    // Mock token last fetch date time to now.
    manager.token_last = SystemTime::now();

    // Delay for 1s to simulate expiration of mocked token.
    tokio::time::delay_for(Duration::from_secs(1)).await;

    // Get new token.
    let token = manager.get_token().await.unwrap();

    // Ensure token isn't the mocked one we defined.
    assert_ne!(token, "test");
  }

  fn get_working_client() -> ClientTokenManager {
    ClientTokenManager::build()
      .domain(&var("AUTH0_DOMAIN").unwrap())
      .audience(&var("AUTH0_AUDIENCE").unwrap())
      .client_id(&var("AUTH0_CLIENT_ID").unwrap())
      .client_secret(&var("AUTH0_CLIENT_SECRET").unwrap())
      .build()
      .unwrap()
  }
}
