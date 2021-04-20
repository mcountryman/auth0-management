use anyhow::Result;

pub mod default;

#[async_trait]
pub trait Auth0Driver: Clone {
  type Error;
  type Method;
  type Builder;

  fn build<U: AsRef<str>>(&self, method: Self::Method, url: U) -> Self::Builder;
}

#[async_trait]
pub trait Auth0Request<D: Auth0Driver> {
  fn build(self, driver: &D) -> Result<D::Builder, D::Error>;
}

#[macro_export]
macro_rules! impl_request {
  ($name:ident, $builder:ident, $response:ident, |$req:ident, $driver:ident| $block:block) => {
    impl_request!($builder, $driver, $response);
    impl_request!($name, |$req, $driver| $block);
  };
  ($builder: ident, $driver:ident, $response:ident) => {
    impl<D: $crate::driver::Auth0Driver> $builder<D> {
      pub fn new(driver: D) -> Self {
        Self::default().driver(driver).clone()
      }
    }

    impl $builder<$crate::driver::default::DefaultAuth0Driver> {
      pub async fn send(self) -> anyhow::Result<$response> {
        use $crate::driver::Auth0Request;

        let req = self.build()?;
        let driver = req.driver.clone();
        let driver = match driver {
          Some(driver) => driver,
          None => anyhow::bail!("Expected driver to be `Some`, got `None`"),
        };

        let builder = req.build(&driver)?;

        Ok(builder.send().await?.json::<$response>().await?)
      }
    }
  };
  ($name:ident, |$req:ident, $driver:ident| $block:block) => {
    #[async_trait]
    impl $crate::driver::Auth0Request<$crate::driver::default::DefaultAuth0Driver>
      for $name<$crate::driver::default::DefaultAuth0Driver>
    {
      fn build(
        self,
        driver: &$crate::driver::default::DefaultAuth0Driver,
      ) -> Result<
        <$crate::driver::default::DefaultAuth0Driver as $crate::driver::Auth0Driver>::Builder,
        <$crate::driver::default::DefaultAuth0Driver as $crate::driver::Auth0Driver>::Error,
      > {
        let $req = self;
        let $driver = driver;

        use reqwest::*;
        use $crate::driver::*;

        Ok($block)
      }
    }
  };
}
