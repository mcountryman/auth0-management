use std::collections::HashMap;

use codegen::{Field, Function, Module, Scope, Struct};
use convert_case::{Case, Casing};

use crate::schemas::{ApiOperation, Items, Manifest, Model, ModelProperty};

lazy_static! {
  pub static ref KEYWORD_MAP: HashMap<String, String> =
    vec![("pub", "public"), ("type", "kind")]
      .into_iter()
      .map(|(key, value)| { (key.to_owned(), value.to_owned()) })
      .collect();
  pub static ref RESERVED_MAP: HashMap<String, String> =
    vec![("string", "String"), ("integer", "i32"),]
      .into_iter()
      .map(|(key, value)| { (key.to_owned(), value.to_owned()) })
      .collect();
}

pub struct Generator {}

impl Generator {
  pub fn new() -> Self {
    Self {}
  }

  pub fn generate(manifests: HashMap<String, Manifest>) {
    let generator = Self::new();
    let mut scope = Scope::new();

    for (name, manifest) in &manifests {
      scope.push_module(generator.build_manifest(name, manifest));
    }

    std::fs::write("auth0-management/src/api.rs", scope.to_string()).unwrap();
  }

  fn build_manifest(&self, path: &str, manifest: &Manifest) -> Module {
    let name = path //
      .replace("/", "")
      .replace(" ", "_")
      .to_case(Case::Snake);
    let mut module = Module::new(&name);

    module.import("serde", "Serialize");
    module.import("serde", "Deserialize");

    for (name, model) in &manifest.models {
      if let Some(model) = self.build_model(name, model) {
        module.push_struct(model);
      }
    }

    module
  }

  fn build_operation(&self, operation: &ApiOperation) -> Function {
    let name = operation.nickname.to_case(Case::Snake);
    let mut func = Function::new(&name);
    let mut docs = vec![operation.summary.clone()];

    func.vis("pub");

    docs.push("".to_string());
    docs.push("# Arguments".to_string());

    for parameter in &operation.parameters {
      if let Some(description) = &parameter.description {
        docs.push(format!("* `{}` - {}", &parameter.name, description));
      } else {
        docs.push(format!("* `{}`", &parameter.name));
      }

      func.arg(&parameter.name, "i32");
    }

    func.doc(&docs.join("\n"));
    func
  }

  fn build_model(&self, name: &str, model: &Model) -> Option<Struct> {
    let parent_name = name.to_case(Case::UpperCamel);
    let mut parent = Struct::new(&parent_name);

    parent
      .vis("pub")
      .derive("Debug")
      .derive("Serialize")
      .derive("Deserialize");

    if let Some(properties) = &model.properties {
      for (name, property) in properties {
        if let Some(field) = self.build_field(name, &parent_name, property) {
          parent.push_field(field);
        }
      }
    }

    Some(parent)
  }

  fn build_field(
    &self,
    name: &str,
    parent_name: &str,
    property: &ModelProperty,
  ) -> Option<Field> {
    let (name_original, name_cased) = self.get_field_name(name);
    let kind = self.get_field_type(&name_cased, parent_name, property);
    let kind = kind.as_ref()?;

    let mut field = Field::new(name_cased.as_str(), kind);

    if let Some(description) = &property.description {
      field.doc(description.split('\n').collect());
    }

    if name_cased != name_original || name_cased == "type" {
      field.annotation(vec![
        format!("#[serde(rename = \"{}\")]", name_original).as_str()
      ]);
    }

    Some(field)
  }

  fn get_field_name(&self, name: &str) -> (String, String) {
    let name_original = name;
    let name_cased = &*name.to_case(Case::Snake);
    let name_cased = if let Some(name_cased) = KEYWORD_MAP.get(name_cased) {
      name_cased.as_str()
    } else {
      name_cased
    };

    (name_original.to_string(), name_cased.to_string())
  }

  fn get_field_type(
    &self,
    name: &str,
    parent_name: &str,
    property: &ModelProperty,
  ) -> Option<String> {
    self.get_type_ref(
      name.to_string(),
      parent_name.to_string(),
      &property.kind,
      &property.items,
      &property.reference,
    )
  }

  fn get_type_ref(
    &self,
    name: String,
    parent_name: String,
    kind: &Option<String>,
    items: &Option<Items>,
    reference: &Option<String>,
  ) -> Option<String> {
    let mut is_array = false;

    if let Some(kind) = kind {
      if kind == "array" {
        is_array = true;
      } else if let Some(kind) = RESERVED_MAP.get(kind) {
        return Some(kind.to_string());
      }
    }

    if let Some(reference) = reference {
      let name = reference.to_case(Case::UpperCamel);
      if is_array {
        return Some(format!("Vec<{}>", name));
      } else {
        return Some(name);
      }
    }

    if let Some(items) = items {}

    None
  }
}
