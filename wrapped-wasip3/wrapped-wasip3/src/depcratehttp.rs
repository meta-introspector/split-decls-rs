// Generated macro for http (module)
macro_rules! Depcratehttp {
() => {
// Module: crate
// Provides: {"http"}
// Dependencies: {}
pub mod http { pub use super :: proxy :: wasi :: http :: * ; pub mod proxy { # [doc = " Generate an exported instance of the `wasi:http/proxy` world."] # [doc = ""] # [doc = " This macro will generate `#[no_mangle]` functions as necessary to"] # [doc = " export an implementation of the"] # [doc = " [`exports::http::handler::Guest`](crate::exports::http::handler::Guest)"] # [doc = " trait. This macro takes an argument which is a type that implements"] # [doc = " this trait:"] # [doc = ""] # [doc = " ```"] # [doc = " use wasip3::http::types::{Request, Response, ErrorCode};"] # [doc = ""] # [doc = " struct MyIncomingHandler;"] # [doc = ""] # [doc = " impl wasip3::exports::http::handler::Guest for MyIncomingHandler {"] # [doc = "     async fn handle(request: Request) -> Result<Response, ErrorCode> {"] # [doc = "         // ..."] # [doc = " # panic!();"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " wasip3::http::proxy::export!(MyIncomingHandler);"] # [doc = " ```"] # [doc = ""] # [doc = " <!--"] # [doc = " The marker above hides the generated documentation by wit-bindgen"] # [doc = " for this macro."] # [doc (inline)] pub use crate :: proxy :: _export_proxy as export ; } }
};
}
