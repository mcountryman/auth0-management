use std::error::Error;
use std::time::{SystemTime, SystemTimeError};

use reqwest::{Client, StatusCode};
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
  Time(SystemTimeError),
  Transport(reqwest::Error),
  AccessDenied(String),
}

#[derive(Serialize, Clone, Debug)]
struct TokenOpts {
  audience: String,
  grant_type: String,
  client_id: String,
  client_secret: String,
}

#[derive(Deserialize, Clone, Debug)]
struct TokenErrorResponse {
  error: String,
  error_description: String,
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
      None => Ok(self.fetch_token().await?),
      Some(token) => {
        let elapsed = SystemTime::now()
          .duration_since(self.token_last)
          .map_err(TokenError::Time)?;

        if elapsed.as_secs() > token.expires_in {
          Ok(self.fetch_token().await?)
        } else {
          Ok(self.token.as_ref().unwrap().access_token.to_owned())
        }
      }
    }
  }

  /// Gets new encoded JWT token from auth0.
  async fn fetch_token(&mut self) -> Result<String, TokenError> {
    let res = self
      .client
      .post(&format!("https://{}/oauth/token", self.domain))
      .form(&self.token_opts)
      .send()
      .await?;

    if res.status() != StatusCode::OK {
      return Err(res.json::<TokenErrorResponse>().await?.into());
    }

    let token = res.json().await?;

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

impl From<TokenErrorResponse> for TokenError {
  fn from(res: TokenErrorResponse) -> TokenError {
    TokenError::AccessDenied(res.error_description)
  }
}

impl From<reqwest::Error> for TokenError {
  fn from(err: reqwest::Error) -> TokenError {
    TokenError::Transport(err)
  }
}

impl Error for TokenError {}
