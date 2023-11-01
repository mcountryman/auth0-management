//! Sorting helper.
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
  field: Option<String>,
  order: Option<Ordering>,
}

impl Sort {
  /// Create field sort.
  pub fn new() -> Self {
    Default::default()
  }

  /// Determines if sort is empty.
  pub fn is_empty(&self) -> bool {
    self.field.is_none() || self.order.is_none()
  }
}

/// Provides sort.
pub trait Sortable {
  /// Sorting strategy.
  ///
  /// # Arguments
  /// * `field` - The field name to sort by. (This will be the name defined by the Auth0 api)
  /// * `order` - The order to sort field values.
  fn sort(&mut self, field: &str, order: Ordering) -> &mut Self;
}

impl<S: AsMut<Sort>> Sortable for S {
  fn sort(&mut self, field: &str, order: Ordering) -> &mut Self {
    self.as_mut().field = Some(field.to_owned());
    self.as_mut().order = Some(order);
    self
  }
}

impl Default for Sort {
  fn default() -> Self {
    Self {
      field: None,
      order: None,
    }
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
    match (&self.field, self.order) {
      (Some(field), Some(order)) => {
        serializer.serialize_str(&format!("{}:{}", field, order as i8))
      }
      _ => serializer.serialize_none(),
    }
  }
}
