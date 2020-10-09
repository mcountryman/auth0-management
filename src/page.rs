//! Paging helper.
use std::ops::DerefMut;

use serde::{Serialize, Serializer};

/// Provides field sort order.
#[derive(Copy, Clone, Serialize)]
pub enum Ordering {
  /// Least to greatest order.
  Ascending = -1,
  /// Greatest to least order.
  Descending = 1,
}

/// Provides field sort.
pub struct Sort {
  field: String,
  order: Ordering,
}

impl Sort {
  /// Create field sort.
  ///
  /// # Arguments
  /// * `field` - The field name to sort by. (This will be the name defined by the Auth0 api)
  /// * `order` - The order to sort field values.
  pub fn new(field: &str, order: Ordering) -> Self {
    Self {
      field: field.to_owned(),
      order,
    }
  }
}

/// Provides serializable pagination parameters.
#[derive(Serialize)]
pub struct Page {
  /// Page index of the results to return.  First page is `0`.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub page: Option<u32>,
  /// Number of results per page.  Paging is disabled if parameter is `Option::None`
  #[serde(rename = "per_page")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub page_size: Option<u32>,
  /// Sorting strategy.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub sort: Option<Sort>,
}

impl Page {
  /// Create pagination instance.
  pub fn new() -> Self {
    Default::default()
  }
}

impl Default for Page {
  fn default() -> Self {
    Self {
      page: None,
      page_size: None,
      sort: None,
    }
  }
}

/// Build pagination parameters.
pub trait PagedBuilder {
  /// Sorting strategy.
  fn sort(&mut self, field: &str, order: Ordering) -> &mut Self;
  /// Page index of the results to return.  First page is `0`.
  fn page(&mut self, page: u32) -> &mut Self;
  /// Number of results per page.  Paging is disabled if parameter is `Option::None`
  fn per_page(&mut self, per_page: u32) -> &mut Self;
}

impl<P: DerefMut<Target = Page>> PagedBuilder for P {
  /// Sorting strategy.
  fn sort(&mut self, field: &str, order: Ordering) -> &mut Self {
    self.deref_mut().sort = Some(Sort::new(field, order));
    self
  }

  /// Page index of the results to return.  First page is `0`.
  fn page(&mut self, page: u32) -> &mut Self {
    self.deref_mut().page = Some(page);
    self
  }

  /// Number of results per page.  Paging is disabled if parameter is `Option::None`
  fn per_page(&mut self, per_page: u32) -> &mut Self {
    self.deref_mut().page_size = Some(per_page);
    self
  }
}

impl Serialize for Sort {
  fn serialize<S>(
    &self,
    serializer: S,
  ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(&format!("{}:{}", self.field, self.order as i8))
  }
}
