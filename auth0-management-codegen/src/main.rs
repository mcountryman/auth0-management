#[macro_use]
extern crate lazy_static;

use rayon::prelude::*;

use crate::generator::Generator;
use crate::schemas::{Manifest, ManifestPath, ManifestRoot};

mod generator;
mod schemas;

const BASE_URL: &str = "https://login.auth0.com/api/v2/api-docs";

fn main() {
  let manifest = ureq::get(BASE_URL) //
    .call()
    .into_json_deserialize::<ManifestRoot>()
    .unwrap();

  Generator::generate(
    manifest //
      .apis
      .par_iter()
      .map(fetch_manifest)
      .collect(),
  );
}

fn fetch_manifest(api: &ManifestPath) -> (String, Manifest) {
  (
    api.path.to_owned(),
    ureq::get(format!("{}{}", BASE_URL, api.path).as_str())
      .call()
      .into_json_deserialize::<Manifest>()
      .expect(format!("api `{}` fetch failed", api.path).as_str()),
  )
}
