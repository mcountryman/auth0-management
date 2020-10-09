//! Retrieve the first confirmed [Guardian](https://auth0.com/docs/multifactor-authentication/guardian)
//! enrollment for a user.

use chrono::{DateTime, Utc};
use reqwest::{Method, RequestBuilder};
use serde::Deserialize;

use crate::Auth0Request;
use crate::User;

/// Retrieve the first confirmed [Guardian](https://auth0
/// .com/docs/multifactor-authentication/guardian)
/// enrollment for a user.
///
/// # Scopes
/// * `read:users`
///
/// # Example
/// ```
/// async fn dump_enrollments() {}
/// ```
#[derive(Debug, Clone, Deserialize)]
pub struct UserEnrollmentsGet {
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

/// Provides data for user enrollments request.
pub struct GetUserEnrollments {
  id: String,
}

impl GetUserEnrollments {
  /// Create user enrollments request.
  pub fn new(id: &str) -> Self {
    Self { id: id.to_owned() }
  }
}

impl<U, A> From<User<U, A>> for GetUserEnrollments {
  fn from(user: User<U, A>) -> Self {
    Self::new(&user.user_id)
  }
}

impl<U, A> From<&User<U, A>> for GetUserEnrollments {
  fn from(user: &User<U, A>) -> Self {
    Self::new(&user.user_id)
  }
}

impl Auth0Request for GetUserEnrollments {
  type Response = Vec<UserEnrollmentsGet>;

  fn build<F>(&self, factory: F) -> RequestBuilder
  where
    F: FnOnce(Method, &str) -> RequestBuilder,
  {
    factory(
      Method::GET,
      &format!("api/v2/users/{}/enrollments", self.id),
    )
  }
}
