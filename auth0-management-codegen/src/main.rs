use rayon::prelude::*;

use crate::jen::Jen;
use crate::schemas::{Manifest, ManifestPath, ManifestRoot};

mod jen;
mod schemas;

const BASE_URL: &str = "https://login.auth0.com/api/v2/api-docs";

fn main() {
  let manifest = ureq::get(BASE_URL) //
    .call()
    .into_json_deserialize::<ManifestRoot>()
    .unwrap();

  manifest //
    .apis
    .par_iter()
    .for_each(handle_api)
}

fn handle_api(api: &ManifestPath) {
  let manifest = ureq::get(format!("{}{}", BASE_URL, api.path).as_str())
    .call()
    .into_json_deserialize::<Manifest>()
    .unwrap();

  Jen::new("target").generate(&manifest);
}
