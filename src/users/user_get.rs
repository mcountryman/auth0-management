//! Retrieve user details. A list of fields to include or exclude may also be specified.
use reqwest::{Method, RequestBuilder};
use serde::de::DeserializeOwned;
use serde::export::PhantomData;

use crate::users::User;
use crate::Auth0RequestBuilder;

/// Retrieve user details. A list of fields to include or exclude may also be specified.
pub struct UserGet<AppMetadata, UserMetadata> {
  id: String,

  app_metadata: PhantomData<AppMetadata>,
  user_metadata: PhantomData<UserMetadata>,
}

impl<AppMetadata, UserMetadata> UserGet<AppMetadata, UserMetadata> {
  /// Create get user request.
  pub fn new<S: AsRef<String>>(id: S) -> Self {
    Self {
      id: id.as_ref().to_string(),
      app_metadata: Default::default(),
      user_metadata: Default::default(),
    }
  }
}

impl<AppMetadata: DeserializeOwned, UserMetadata: DeserializeOwned> Auth0RequestBuilder
  for UserGet<AppMetadata, UserMetadata>
{
  type Response = User<AppMetadata, UserMetadata>;

  fn build<F>(&self, factory: F) -> RequestBuilder
  where
    F: FnOnce(Method, &str) -> RequestBuilder,
  {
    factory(Method::GET, &format!("api/v2/users/{}", self.id))
  }
}

/// Provides data used to request user from email field.
pub struct GetUserByEmail<AppMetadata, UserMetadata> {
  email: String,

  app_metadata: PhantomData<AppMetadata>,
  user_metadata: PhantomData<UserMetadata>,
}

impl<AppMetadata, UserMetadata> GetUserByEmail<AppMetadata, UserMetadata> {
  /// Create get user request.
  /// # Arguments
  /// * `email` - The email address of the user to retrieve.
  pub fn new(email: &str) -> Self {
    Self {
      email: email.to_owned(),
      app_metadata: Default::default(),
      user_metadata: Default::default(),
    }
  }
}

impl<AppMetadata: DeserializeOwned, UserMetadata: DeserializeOwned> Auth0RequestBuilder
  for GetUserByEmail<AppMetadata, UserMetadata>
{
  type Response = User<AppMetadata, UserMetadata>;

  fn build<F>(&self, factory: F) -> RequestBuilder
  where
    F: FnOnce(Method, &str) -> RequestBuilder,
  {
    factory(Method::GET, "api/v2/users/users-by-email")
      .query(&[("email", self.email.to_owned())])
  }
}
