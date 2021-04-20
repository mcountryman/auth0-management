#![deny(unsafe_code)]

#[macro_use]
extern crate serde;
#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate derive_builder;

pub mod api;
pub mod driver;

use driver::Auth0Driver;

#[derive(Clone)]
pub struct Auth0ManagementClient<D: Auth0Driver> {
  driver: D,
}

impl<D: Auth0Driver> Auth0ManagementClient<D> {}
