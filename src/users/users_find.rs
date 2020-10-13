//! Retrieve details of users.
use reqwest::{Method, RequestBuilder};
use serde::de::DeserializeOwned;
use serde::export::PhantomData;
use serde::Serialize;

use crate::users::User;
use crate::{Auth0Client, Auth0RequestBuilder};
use crate::{Page, Sort};

/// Retrieve details of users.
#[derive(Serialize)]
pub struct UsersFind<'a, A, U> {
  #[serde(skip_serializing)]
  client: &'a Auth0Client,

  #[serde(flatten)]
  page: Page,
  #[serde(skip_serializing_if = "Sort::is_emtpy")]
  sort: Sort,

  app_metadata: PhantomData<A>,
  user_metadata: PhantomData<U>,
}

impl<'a, A, U> UsersFind<'a, A, U> {
  /// Create find users request.
  pub fn new(client: &'a Auth0Client) -> Self {
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

impl<'a, A, U> AsRef<Auth0Client> for UsersFind<'a, A, U> {
  fn as_ref(&self) -> &Auth0Client {
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
