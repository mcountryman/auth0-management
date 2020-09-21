use codegen::Scope;
use convert_case::{Case, Casing};
use rayon::prelude::*;

use crate::schemas::{Manifest, Model};
use std::cell::RefCell;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

pub struct Jen {
  target: PathBuf,
}

impl Jen {
  pub fn new(target: &str) -> Self {
    Self {
      target: PathBuf::from(target),
    }
  }

  pub fn generate(&self, manifest: &Manifest) {
    self.generate_models(manifest);
  }

  fn generate_models(&self, manifest: &Manifest) {
    let scope = RefCell::new(Scope::new());

    manifest.models.iter().for_each(|(name, model)| {
      self.generate_model(scope.clone(), name, model);
    });

    std::fs::create_dir_all(format!("{}/{}", self.target, manifest.base_path));
    std::fs::write("")
  }

  fn generate_model(&self, scope: RefCell<Scope>, name: &String, model: &Model) {
    let mut scope = scope.into_inner();
    let model_struct = scope
      .new_struct(&*name.to_case(Case::UpperCamel))
      .derive("Debug")
      .derive("serde::Serialize")
      .derive("serde::Deserialize");

    if let Some(properties) = &model.properties {
      for (name, _property) in properties {
        model_struct.field(&*name.to_case(Case::Snake), "i32");
      }
    }
  }
}
