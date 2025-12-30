// Generated macro for Client (struct)
macro_rules! Depcrate_clientClient {
() => {
// Module: crate::client
// Provides: {"Client"}
// Dependencies: {}
# [doc = " An HTTP client, capable of sending `Request`s and running a middleware stack."] # [doc = ""] # [doc = " Can be optionally set with a base url."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # #[async_std::main]"] # [doc = " # async fn main() -> surf::Result<()> {"] # [doc = " let client = surf::Client::new();"] # [doc = " let res1 = client.recv_string(surf::get(\"https://httpbin.org/get\"));"] # [doc = " let res2 = client.recv_string(surf::get(\"https://httpbin.org/get\"));"] # [doc = " let (str1, str2) = futures_util::future::try_join(res1, res2).await?;"] # [doc = " # Ok(()) }"] # [doc = " ```"] pub struct Client { config : Config , http_client : Arc < dyn HttpClient > , # [doc = " Holds the middleware stack."] # [doc = ""] # [doc = " Note(Fishrock123): We do actually want this structure."] # [doc = " The outer Arc allows us to clone in .send() without cloning the array."] # [doc = " The Vec allows us to add middleware at runtime."] # [doc = " The inner Arc-s allow us to implement Clone without sharing the vector with the parent."] # [doc = " We don't use a Mutex around the Vec here because adding a middleware during execution should be an error."] # [allow (clippy :: rc_buffer)] middleware : Arc < Vec < Arc < dyn Middleware > > > , }
};
}
