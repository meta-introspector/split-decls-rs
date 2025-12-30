// Generated macro for Config (struct)
macro_rules! Depcrate_configConfig {
() => {
// Module: crate::config
// Provides: {"Config"}
// Dependencies: {}
# [doc = " Configuration for `surf::Client`s and their underlying HTTP clients."] # [doc = ""] # [doc = " ```"] # [doc = " use std::convert::TryInto;"] # [doc = " use surf::{Client, Config, Url};"] # [doc = ""] # [doc = " # #[async_std::main]"] # [doc = " # async fn main() -> surf::Result<()> {"] # [doc = " let client: Client = Config::new()"] # [doc = "     .set_base_url(Url::parse(\"https://example.org\")?)"] # [doc = "     .try_into()?;"] # [doc = ""] # [doc = " let mut response = client.get(\"/\").await?;"] # [doc = ""] # [doc = " println!(\"{}\", response.body_string().await?);"] # [doc = " # Ok(())"] # [doc = " # }"] # [doc = " ```"] # [non_exhaustive] # [derive (Clone , Debug)] pub struct Config { # [doc = " The base URL for a client. All request URLs will be relative to this URL."] # [doc = ""] # [doc = " Note: a trailing slash is significant."] # [doc = " Without it, the last path component is considered to be a “file” name"] # [doc = " to be removed to get at the “directory” that is used as the base."] pub base_url : Option < Url > , # [doc = " Headers to be applied to every request made by this client."] pub headers : HashMap < HeaderName , HeaderValues > , # [doc = " Underlying HTTP client config."] pub http_config : HttpConfig , # [doc = " Optional custom http client."] pub http_client : Option < Arc < dyn HttpClient > > , }
};
}
