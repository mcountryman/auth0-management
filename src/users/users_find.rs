//! Retrieve details of users.
use reqwest::{Method, RequestBuilder};
use serde::de::DeserializeOwned;
use serde::export::PhantomData;
use serde::Serialize;

use crate::users::User;
use crate::{Auth0, Auth0RequestBuilder};
use crate::{Page, Sort};

/// Retrieve details of users. It is possible to:
///
/// - Specify a search criteria for users
/// - Sort the users to be returned
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
pub struct UsersFind<'a, A, U> {
  #[serde(skip_serializing)]
  client: &'a Auth0,

  #[serde(flatten)]
  page: Page,
  #[serde(skip_serializing_if = "Sort::is_emtpy")]
  sort: Sort,

  app_metadata: PhantomData<A>,
  user_metadata: PhantomData<U>,
}

impl<'a, A, U> UsersFind<'a, A, U> {
  /// Create find users request.
  pub fn new(client: &'a Auth0) -> Self {
    Self {
      client,
      
      page: Default::default(),
      sort: Default::default(),

      app_metadata: Default::default(),
      user_metadata: Default::default(),
    }
  }
}

impl<'a, A, U> AsMut<Page> for UsersFind<'a, A, U> {
  fn as_mut(&mut self) -> &mut Page {
    &mut self.page
  }
}

impl<'a, A, U> AsMut<Sort> for UsersFind<'a, A, U> {
  fn as_mut(&mut self) -> &mut Sort {
    &mut self.sort
  }
}

impl<'a, A, U> AsRef<Auth0> for UsersFind<'a, A, U> {
  fn as_ref(&self) -> &Auth0 {
    self.client
  }
}

impl<'a, A: DeserializeOwned, U: DeserializeOwned> Auth0RequestBuilder
  for UsersFind<'a, A, U>
{
  type Response = Vec<User<A, U>>;

  fn build<F>(&self, factory: F) -> RequestBuilder
  where
    F: FnOnce(Method, &str) -> RequestBuilder,
  {
    factory(Method::GET, "api/v2/users").query(self)
  }
}
