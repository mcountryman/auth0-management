use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Strategy {
  #[serde(alias = "ad")]
  ActiveDirectory,
  #[serde(alias = "adfs")]
  ActiveDirectoryFederationServices,
  #[serde(alias = "amazon")]
  Amazon,
  #[serde(alias = "apple")]
  Apple,
  #[serde(alias = "dropbox")]
  Dropbox,
  #[serde(alias = "bitbucket")]
  Bitbucket,
  #[serde(alias = "aol")]
  AOL,
  #[serde(alias = "auth0-oidc")]
  Auth0OpenIdConnect,
  #[serde(alias = "auth0")]
  Auth0,
  #[serde(alias = "baidu")]
  Baidu,
  #[serde(alias = "bitly")]
  Bitly,
  #[serde(alias = "box")]
  Box,
  #[serde(alias = "custom")]
  Custom,
  #[serde(alias = "daccount")]
  DAccount,
  #[serde(alias = "dwolla")]
  DWolla,
  #[serde(alias = "email")]
  Email,
  #[serde(alias = "evernote-sandbox")]
  EvernoteSandbox,
  #[serde(alias = "evernote")]
  Evernote,
  #[serde(alias = "exact")]
  Exact,
  #[serde(alias = "facebook")]
  Facebook,
  #[serde(alias = "fitbit")]
  Fitbit,
  #[serde(alias = "flickr")]
  Flickr,
  #[serde(alias = "github")]
  Github,
  #[serde(alias = "google-apps")]
  GoogleApps,
  #[serde(alias = "google-oauth2")]
  GoogleOAuth2,
  #[serde(alias = "instagram")]
  Instagram,
  #[serde(alias = "ip")]
  IP,
  #[serde(alias = "linkedin")]
  LinkedIn,
  #[serde(alias = "miicard")]
  MiiCard,
  #[serde(alias = "oauth1")]
  OAuth1,
  #[serde(alias = "oauth2")]
  OAuth2,
  #[serde(alias = "office365")]
  Office365,
  #[serde(alias = "oidc")]
  OpenIdConnect,
  #[serde(alias = "paypal")]
  Paypal,
  #[serde(alias = "paypal-sandbox")]
  PaypalSandbox,
  #[serde(alias = "pingfederate")]
  PingFederate,
  #[serde(alias = "planningcenter")]
  PlanningCenter,
  #[serde(alias = "renren")]
  Renren,
  #[serde(alias = "salesforce-community")]
  SalesforceCommunity,
  #[serde(alias = "salesforce-sandbox")]
  SalesforceSandbox,
  #[serde(alias = "salesforce")]
  SalesForce,
  #[serde(alias = "samlp")]
  Samlp,
  #[serde(alias = "sharepoint")]
  Sharepoint,
  #[serde(alias = "shopify")]
  Shopify,
  #[serde(alias = "sms")]
  Sms,
  #[serde(alias = "soundcloud")]
  Soundcloud,
  #[serde(alias = "thecity-sandbox")]
  TheCitySandbox,
  #[serde(alias = "thecity")]
  TheCity,
  #[serde(alias = "thirtysevensignals")]
  ThirtySevenSignals,
  #[serde(alias = "twitter")]
  Twitter,
  #[serde(alias = "untappd")]
  Untappd,
  #[serde(alias = "vkontakte")]
  Vkontakte,
  #[serde(alias = "waad")]
  Waad,
  #[serde(alias = "weibo")]
  Weibo,
  #[serde(alias = "windowslive")]
  WindowsLive,
  #[serde(alias = "wordpress")]
  Wordpress,
  #[serde(alias = "yahoo")]
  Yahoo,
  #[serde(alias = "yammer")]
  Yammer,
  #[serde(alias = "yandex")]
  Yandex,
  #[serde(alias = "line")]
  Line,
}
