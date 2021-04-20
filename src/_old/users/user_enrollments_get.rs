//! Retrieve the first confirmed [Guardian](https://auth0.com/docs/multifactor-authentication/guardian)
//! enrollment for a user.

use chrono::{DateTime, Utc};
use reqwest::{Method, RequestBuilder};
use serde::Deserialize;

use crate::{Auth0Client, Auth0RequestBuilder};

/// Multi-factor enrollment.
#[derive(Debug, Clone, Deserialize)]
pub struct UserEnrollment {
  /// ID of this enrollment.
  pub id: String,
  /// Status of this enrollment. Can be `pending` or `confirmed`.
  pub status: String,
  /// Type of enrollment.
  #[serde(rename = "type")]
  pub kind: String,
  /// Name of enrollment (usually phone number).
  pub name: String,
  /// Device identifier (usually phone identifier) of this enrollment.
  pub identifier: String,
  /// Phone number for this enrollment.
  pub phone_number: String,
  /// Authentication method for this enrollment. Can be `authentication`, `guardian`, or
  /// `sms`.
  pub auth_method: String,
  /// Start date and time of this enrollment.
  pub enrolled_at: DateTime<Utc>,
  /// Last authentication date and time of this enrollment.
  pub last_auth: DateTime<Utc>,
}

/// Retrieve the first confirmed [Guardian](https://auth0
/// .com/docs/multifactor-authentication/guardian)
/// enrollment for a user.
pub struct UserEnrollmentsGet {
  id: String,
}

impl UserEnrollmentsGet {
  /// Create user enrollments request.
  pub fn new<S: AsRef<str>>(id: S) -> Self {
    Self {
      id: id.as_ref().to_owned(),
    }
  }
}

impl Auth0RequestBuilder for UserEnrollmentsGet {
  fn build(&self, client: &Auth0Client) -> RequestBuilder {
    client.begin(
      Method::GET,
      &format!("api/v2/users/{}/enrollments", self.id),
    )
  }
}
