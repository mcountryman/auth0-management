//! Retrieve details of users.
use reqwest::{Method, RequestBuilder};
use serde::de::DeserializeOwned;
use serde::export::PhantomData;
use serde::Serialize;

use crate::users::{EmptyAppMetadata, EmptyUserMetadata, User};
use crate::Auth0Request;
use crate::{Page, Sort};

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
  #[serde(skip_serializing_if = "Sort::is_emtpy")]
  sort: Sort,

  app_metadata: PhantomData<AppMetadata>,
  user_metadata: PhantomData<UserMetadata>,
}

impl<A, U> UsersFind<A, U> {
  /// Create find users request.
  pub fn new() -> Self {
    Self {
      page: Default::default(),
      sort: Default::default(),

      app_metadata: Default::default(),
      user_metadata: Default::default(),
    }
  }
}

impl<AppMetadata, UserMetadata> AsMut<Page> for UsersFind<AppMetadata, UserMetadata> {
  fn as_mut(&mut self) -> &mut Page {
    &mut self.page
  }
}

impl<AppMetadata, UserMetadata> AsMut<Sort> for UsersFind<AppMetadata, UserMetadata> {
  fn as_mut(&mut self) -> &mut Sort {
    &mut self.sort
  }
}

impl<AppMetadata, UserMetadata> Default for UsersFind<AppMetadata, UserMetadata> {
  fn default() -> Self {
    Self {
      page: Default::default(),
      sort: Default::default(),

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
