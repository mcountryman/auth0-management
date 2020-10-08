use reqwest::{Method, RequestBuilder};
use serde::de::DeserializeOwned;
use serde::export::PhantomData;
use serde::Serialize;

use crate::api::page::{Ordering, Page, PagedBuilder, Sort};
use crate::api::users::{EmptyAppMetadata, EmptyUserMetadata, User};
use crate::request::Auth0Request;
use std::ops::{Deref, DerefMut};

/// Provide data to find users.
#[derive(Serialize)]
pub struct FindUsers<AppMetadata = EmptyAppMetadata, UserMetadata = EmptyUserMetadata> {
  #[serde(flatten)]
  page: Page,

  app_metadata: PhantomData<AppMetadata>,
  user_metadata: PhantomData<UserMetadata>,
}

impl<AppMetadata, UserMetadata> FindUsers<AppMetadata, UserMetadata> {
  /// Create find users request.
  pub fn new() -> Self {
    Default::default()
  }
}

impl<AppMetadata, UserMetadata> Deref for FindUsers<AppMetadata, UserMetadata> {
  type Target = Page;

  fn deref(&self) -> &Self::Target {
    &self.page
  }
}

impl<AppMetadata, UserMetadata> DerefMut for FindUsers<AppMetadata, UserMetadata> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.page
  }
}

impl<AppMetadata, UserMetadata> Default for FindUsers<AppMetadata, UserMetadata> {
  fn default() -> Self {
    Self {
      page: Default::default(),

      app_metadata: Default::default(),
      user_metadata: Default::default(),
    }
  }
}

impl<AppMetadata: DeserializeOwned, UserMetadata: DeserializeOwned> Auth0Request
  for FindUsers<AppMetadata, UserMetadata>
{
  type Response = Vec<User<AppMetadata, UserMetadata>>;

  fn build<F>(&self, factory: F) -> RequestBuilder
  where
    F: FnOnce(Method, &str) -> RequestBuilder,
  {
    factory(Method::GET, "api/v2/users").query(self)
  }
}
