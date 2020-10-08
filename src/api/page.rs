use serde::{Deserialize, Serialize, Serializer};

#[derive(Copy, Clone)]
pub enum Order {
  Ascending = -1,
  Descending = 1,
}

pub struct Sort {
  field: String,
  order: Order,
}

impl Sort {
  pub fn new(field: &str, order: Order) -> Self {
    Self {
      field: field.to_owned(),
      order,
    }
  }
}

#[derive(Serialize)]
pub struct Page {
  #[serde(skip_serializing_if = "Option::is_none")]
  page: Option<u32>,
  #[serde(rename = "per_page")]
  #[serde(skip_serializing_if = "Option::is_none")]
  page_size: Option<u32>,
  #[serde(flatten)]
  #[serde(skip_serializing_if = "Option::is_none")]
  sort: Option<Sort>,
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

/// Provides entity with the ability to modified paged data.
pub trait Paged {
  fn page(&mut self) -> &mut Page;
}

pub trait PagedBuilder {
  /// The results sort.
  fn sort(&mut self, field: &str, order: Order) -> &mut Self;
  /// The page index of the results to return. First page is 0.
  fn page(&mut self, page: u32) -> &mut Self;
  /// The number of results per page. Paging is disabled if parameter not sent.
  fn per_page(&mut self, per_page: u32) -> &mut Self;
}

impl<P: Paged> PagedBuilder for P {
  /// The results sort.
  fn sort(&mut self, field: &str, order: Order) -> &mut Self {
    self.page().sort = Some(Sort::new(field, order));
    self
  }

  /// The page index of the results to return. First page is 0.
  fn page(&mut self, page: u32) -> &mut Self {
    self.page().page = Some(page);
    self
  }

  /// The number of results per page. Paging is disabled if parameter not sent.
  fn per_page(&mut self, per_page: u32) -> &mut Self {
    self.page().page_size = Some(per_page);
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
