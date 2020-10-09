//! Retrieve details of users.
use reqwest::{Method, RequestBuilder};
use serde::de::DeserializeOwned;
use serde::export::PhantomData;
use serde::Serialize;
use std::ops::{Deref, DerefMut};

use crate::request::Auth0Request;
use crate::users::{EmptyAppMetadata, EmptyUserMetadata, User};
use crate::Page;

/// Retrieve details of users. It is possible to:
///
/// - Specify a search criteria for users
/// - Sort the users to be returned
/// - Select the fields to be returned
/// - Specify the number of users to retrieve per page and the page index
///
/// The q query parameter can be used to get users that match the specified criteria using
/// [query string syntax](https://auth0.com/docs/users/search/v3/query-syntax).
///
/// [Learn more about searching for users.](https://auth0.com/docs/users/search/v3)
///
/// Read about [best practices](https://auth0.com/docs/users/search/best-practices) when working
/// with the API endpoints for retrieving users.
///
/// Auth0 limits the number of users you can return. If you exceed this threshold, please
/// redefine your search, use the
/// [export job](https://auth0.com/docs/api/management/v2#!/Jobs/post_users_exports), or the
/// [User Import / Export](https://auth0.com/docs/extensions/user-import-export) extension.
///
/// # Scopes
/// * `read:users`
/// * `read:user_idp_tokens`
#[derive(Serialize)]
pub struct UsersFind<AppMetadata = EmptyAppMetadata, UserMetadata = EmptyUserMetadata> {
  #[serde(flatten)]
  page: Page,

  app_metadata: PhantomData<AppMetadata>,
  user_metadata: PhantomData<UserMetadata>,
}

impl<AppMetadata, UserMetadata> UsersFind<AppMetadata, UserMetadata> {
  /// Create find users request.
  pub fn new() -> Self {
    Default::default()
  }
}

impl<AppMetadata, UserMetadata> Deref for UsersFind<AppMetadata, UserMetadata> {
  type Target = Page;

  fn deref(&self) -> &Self::Target {
    &self.page
  }
}

impl<AppMetadata, UserMetadata> DerefMut for UsersFind<AppMetadata, UserMetadata> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.page
  }
}

impl<AppMetadata, UserMetadata> Default for UsersFind<AppMetadata, UserMetadata> {
  fn default() -> Self {
    Self {
      page: Default::default(),

      app_metadata: Default::default(),
      user_metadata: Default::default(),
    }
  }
}

impl<AppMetadata: DeserializeOwned, UserMetadata: DeserializeOwned> Auth0Request
  for UsersFind<AppMetadata, UserMetadata>
{
  type Response = Vec<User<AppMetadata, UserMetadata>>;

  fn build<F>(&self, factory: F) -> RequestBuilder
  where
    F: FnOnce(Method, &str) -> RequestBuilder,
  {
    factory(Method::GET, "api/v2/users").query(self)
  }
}
