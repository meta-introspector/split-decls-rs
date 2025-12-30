// Generated macro for macro_133 (macro)
macro_rules! Depcratemacro_133 {
() => {
// Module: crate
// Provides: {"macro_133"}
// Dependencies: {}
cfg_if :: cfg_if ! { if # [cfg (feature = "default-client")] { mod one_off ; pub use one_off :: { connect , delete , get , head , options , patch , post , put , trace } ; # [doc = " Construct a new `Client`, capable of sending `Request`s and running a middleware stack."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # #[async_std::main]"] # [doc = " # async fn main() -> surf::Result<()> {"] # [doc = " let client = surf::client();"] # [doc = ""] # [doc = " let req = surf::get(\"https://httpbin.org/get\");"] # [doc = " let res = client.send(req).await?;"] # [doc = " # Ok(()) }"] # [doc = " ```"] pub fn client () -> Client { Client :: new () } } }
};
}
