//! Builder for Auth0.
use std::error::Error;
use std::fmt::{Display, Formatter};

use reqwest::Client;

use crate::rate::RateLimit;
use crate::token::TokenManager;
use crate::{Auth0, Auth0Client};

/// Management client interface.
pub struct Auth0Builder {
  domain: Option<String>,
  audience: Option<String>,
  client_id: Option<String>,
  client_secret: Option<String>,
}

impl Auth0Builder {
  /// Get instance of management client builder.
  pub fn new() -> Self {
    Default::default()
  }

  /// Get instance of management client.
  ///
  /// Creates instance of management client and validates builder options.  Valid builder options
  /// requires all fields to be populated.
  pub fn build(self) -> Result<Auth0, Auth0BuilderError> {
    let client = Client::new();
    let domain = self.domain.ok_or(Auth0BuilderError::MissingDomain)?;
    let audience = self.audience.ok_or(Auth0BuilderError::MissingAudience)?;
    let client_id = self.client_id.ok_or(Auth0BuilderError::MissingClientID)?;
    let client_secret = self
      .client_secret
      .ok_or(Auth0BuilderError::MissingClientSecret)?;
    let client = Auth0Client::new(
      RateLimit::new(),
      TokenManager::new(
        client.clone(),
        &domain.clone(),
        &audience,
        &client_id,
        &client_secret,
      ),
      client,
      &domain,
    );

    Ok(Auth0::new(client))
  }

  /// The auth0 tenant domain.
  ///
  /// The domain can be found in the Auth0 dashboard under your application settings.  Domain
  /// will be the field labeled `Domain`.  Domains will be typically formatted as
  /// `example.[us|eu|au].auth0.com`.
  pub fn domain(mut self, domain: &str) -> Self {
    self.domain = Some(domain.to_owned());
    self
  }

  /// The value of the Identifier field of the `Auth0 Management API`.  
  ///
  /// The audience can be found in the Auth0 dashboard under you `API` settings.  Audience will be
  /// in the field labeled `Identifier`.  The default management API identifier will be formatted as
  /// `https://example.eu.auth0.com/api/v2`
  pub fn audience(mut self, audience: &str) -> Self {
    self.audience = Some(audience.to_owned());
    self
  }

  /// The value of the Client ID field of the Machine-to-Machine application.
  ///
  /// The client id can be found in the Auth0 dashboard under your application settings.  Client ID
  /// will be the field labeled `Client ID`.
  pub fn client_id(mut self, client_id: &str) -> Self {
    self.client_id = Some(client_id.to_owned());
    self
  }

  /// The value of the Client Secret field of the Machine-to-Machine application.
  ///
  /// The client secret can be found in the Auth0 dashboard under your application settings.  
  /// Client Secret will be the field labeled `Client Secret`.
  pub fn client_secret(mut self, client_secret: &str) -> Self {
    self.client_secret = Some(client_secret.to_owned());
    self
  }
}

impl Default for Auth0Builder {
  fn default() -> Self {
    Self {
      domain: None,
      audience: None,
      client_id: None,
      client_secret: None,
    }
  }
}

/// The error type which is returned from building a [Auth0].
#[derive(Debug, PartialOrd, PartialEq)]
pub enum Auth0BuilderError {
  /// Indicates builder didn't set [Auth0Builder::domain].
  MissingDomain,
  /// Indicates builder didn't set [Auth0Builder::audience].
  MissingAudience,
  /// Indicates builder didn't set [Auth0Builder::client_id].
  MissingClientID,
  /// Indicates builder didn't set [Auth0Builder::client_secret].
  MissingClientSecret,
}

impl Display for Auth0BuilderError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl Error for Auth0BuilderError {}
