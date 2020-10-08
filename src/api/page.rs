use serde::{Deserialize, Serialize, Serializer};
use std::ops::DerefMut;

#[derive(Copy, Clone)]
pub enum Ordering {
  Ascending = -1,
  Descending = 1,
}

pub struct Sort {
  field: String,
  order: Ordering,
}

impl Sort {
  pub fn new(field: &str, order: Ordering) -> Self {
    Self {
      field: field.to_owned(),
      order,
    }
  }
}

#[derive(Serialize)]
pub struct Page {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub page: Option<u32>,
  #[serde(rename = "per_page")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub page_size: Option<u32>,
  #[serde(flatten)]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub sort: Option<Sort>,
}

impl Page {
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

pub trait PagedBuilder {
  /// The results sort.
  fn sort(&mut self, field: &str, order: Ordering) -> &mut Self;
  /// The page index of the results to return. First page is 0.
  fn page(&mut self, page: u32) -> &mut Self;
  /// The number of results per page. Paging is disabled if parameter not sent.
  fn per_page(&mut self, per_page: u32) -> &mut Self;
}

impl<P: DerefMut<Target = Page>> PagedBuilder for P {
  /// The results sort.
  fn sort(&mut self, field: &str, order: Ordering) -> &mut Self {
    self.deref_mut().sort = Some(Sort::new(field, order));
    self
  }

  /// The page index of the results to return. First page is 0.
  fn page(&mut self, page: u32) -> &mut Self {
    self.deref_mut().page = Some(page);
    self
  }

  /// The number of results per page. Paging is disabled if parameter not sent.
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
