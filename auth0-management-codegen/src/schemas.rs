use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct ManifestRoot {
  #[serde(alias = "apiVersion")]
  pub api_version: String,
  #[serde(alias = "swaggerVersion")]
  pub swagger_version: String,
  pub apis: Vec<ManifestPath>,
}

#[derive(Deserialize, Debug)]
pub struct ManifestPath {
  pub path: String,
}

#[derive(Deserialize, Debug)]
pub struct Manifest {
  #[serde(alias = "apiVersion")]
  pub api_version: String,
  #[serde(alias = "swaggerVersion")]
  pub swagger_version: String,
  #[serde(alias = "basePath")]
  pub base_path: String,
  #[serde(alias = "resourcePath")]
  pub resource_path: String,

  pub apis: Vec<Api>,
  pub models: HashMap<String, Model>,
}

#[derive(Deserialize, Debug)]
pub struct Api {
  pub path: String,
  pub operations: Vec<ApiOperation>,
}

#[derive(Deserialize, Debug)]
pub struct ApiOperation {
  #[serde(rename = "type")]
  pub kind: Option<String>,
  pub method: String,

  pub notes: Option<String>,
  pub summary: String,
  pub nickname: String,
  pub parameters: Vec<ApiOperationParameter>,
  pub items: Option<Items>,
  #[serde(alias = "responseMessages")]
  pub messages: Option<Vec<ApiOperationMessage>>,
}

#[derive(Deserialize, Debug)]
pub struct ApiOperationParameter {
  pub name: String,
  #[serde(rename = "type")]
  pub kind: String,
  pub minimum: Option<i32>,
  pub maximum: Option<i32>,
  pub description: Option<String>,
  #[serde(rename = "paramType")]
  pub param_type: String,
}

#[derive(Deserialize, Debug)]
pub struct ApiOperationMessage {
  pub code: Option<ApiOperationMessageCode>,
  pub message: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ApiOperationMessageCode {
  Number(i32),
  String(String),
}

#[derive(Deserialize, Debug)]
pub struct Model {
  #[serde(rename = "type")]
  pub kind: Option<String>,
  pub properties: Option<HashMap<String, ModelProperty>>,
}

#[derive(Deserialize, Debug)]
pub struct ModelProperty {
  #[serde(rename = "$ref")]
  pub reference: Option<String>,
  #[serde(rename = "type")]
  pub kind: Option<String>,
  pub items: Option<Items>,
  pub format: Option<String>,
  pub description: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Items {
  #[serde(rename = "$ref")]
  pub reference: Option<String>,
  #[serde(rename = "type")]
  pub kind: Option<String>,
  #[serde(rename = "minLength")]
  pub min_length: Option<i32>,
  #[serde(rename = "maxLength")]
  pub max_length: Option<i32>,
  pub properties: Option<HashMap<String, ModelProperty>>,
}
