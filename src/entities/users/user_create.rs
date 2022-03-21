//! User create request.

use super::user::User;
use crate::{
  error::Auth0Result,
  http::{HttpMethod, HttpRequest},
  Auth0Request,
};
use derive_builder::Builder;
use serde::Serialize;
use std::sync::Arc;

/// Create a new user for a given [database](https://auth0.com/docs/connections/database) or
/// [passwordless](https://auth0.com/docs/connections/passwordless) connection.
#[derive(Serialize, Clone, Debug, Builder)]
pub struct UserCreate<Client, A, U> {
  /// The client.
  #[serde(skip_serializing)]
  client: Arc<Client>,

  /// A.
  #[builder(setter(into, strip_option))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub email: Option<String>,
  /// A.
  #[builder(setter(into, strip_option))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub phone_number: Option<String>,
  /// A.
  #[builder(setter(into, strip_option))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub blocked: Option<bool>,
  /// A.
  #[builder(setter(into, strip_option))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub email_verified: Option<bool>,
  /// A.
  #[builder(setter(into, strip_option))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub phone_verified: Option<bool>,
  /// A.
  #[builder(setter(into, strip_option))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub given_name: Option<String>,
  /// A.
  #[builder(setter(into, strip_option))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub family_name: Option<String>,
  /// A.
  #[builder(setter(into, strip_option))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  /// A.
  #[builder(setter(into, strip_option))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub nickname: Option<String>,
  /// A.
  #[builder(setter(into, strip_option))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub picture: Option<String>,
  /// A.
  #[builder(setter(into, strip_option))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub user_id: Option<String>,
  /// A.
  #[builder(setter(into, strip_option))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub connection: Option<String>,
  /// A.
  #[builder(setter(into, strip_option))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub password: Option<String>,
  /// A.
  #[builder(setter(into, strip_option))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub verify_email: Option<bool>,
  /// A.
  #[builder(setter(into, strip_option))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub username: Option<String>,

  /// A.
  #[builder(setter(into, strip_option))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub app_metadata: Option<A>,
  /// A.
  #[builder(setter(into, strip_option))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub user_metadata: Option<U>,
}

impl<Client, A, U> Auth0Request<Client> for UserCreateBuilder<Client, A, U>
where
  A: Clone,
  U: Clone,
  Client: Clone,
{
  type Body = UserCreate<Client, A, U>;
  type Response = User<A, U>;

  fn client(&self) -> Auth0Result<Arc<Client>> {
    Ok(self.client.as_ref().unwrap().clone())
  }

  fn to_request(&self) -> Auth0Result<HttpRequest<Self::Body>> {
    Ok(
      HttpRequest::builder()
        .method(HttpMethod::Post)
        .path("/api/v2/users")
        .body(self.build()?)
        .build()?,
    )
  }
}

// impl<A, U> TryIntoRequest<HttpAuth0Client> for UserCreateBuilder<HttpAuth0Client, A, U>
// where
//   A: Send + Sync + Clone + DeserializeOwned,
//   U: Send + Sync + Clone + DeserializeOwned,
// {
//   type Res = User<A, U>;
//   type Req = UserCreate<HttpAuth0Client, A, U>;
//   type Error = ();

//   fn try_into_request(&self) -> Result<Self::Req, Self::Error> {
//     todo!()
//   }
// }

// #[async_trait::async_trait]
// impl<A, U> Auth0Request<User<A, U>, HttpAuth0Client> for UserCreate<HttpAuth0Client, A, U>
// where
//   A: Send + Sync + Clone + Serialize + DeserializeOwned,
//   U: Send + Sync + Clone + Serialize + DeserializeOwned,
// {
//   async fn send(&self) -> Result<User<A, U>, reqwest::Error> {
//     self
//       .client
//       .post("/api/v2/users")
//       .json(self)
//       .send()
//       .await?
//       .json()
//       .await
//   }
// }
