// Generated macro for http (module)
macro_rules! Depcratehttp {
() => {
// Module: crate
// Provides: {"http"}
// Dependencies: {}
pub mod http { pub use super :: proxy :: wasi :: http :: * ; pub mod proxy { # [doc = " Generate an exported instance of the `wasi:http/proxy` world."] # [doc = ""] # [doc = " This macro will generate `#[no_mangle]` functions as necessary to"] # [doc = " export an implementation of the"] # [doc = " [`exports::http::incoming_handler::Guest`](crate::exports::http::incoming_handler::Guest)"] # [doc = " trait.  This macro takes"] # [doc = " an argument which is a type that implements this trait:"] # [doc = ""] # [doc = " ```"] # [doc = " use wasip2::http::types::{IncomingRequest, ResponseOutparam};"] # [doc = ""] # [doc = " struct MyIncomingHandler;"] # [doc = ""] # [doc = " impl wasip2::exports::http::incoming_handler::Guest for MyIncomingHandler {"] # [doc = "     fn handle(request: IncomingRequest, response_out: ResponseOutparam) {"] # [doc = "         // ..."] # [doc = " # panic!();"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " wasip2::http::proxy::export!(MyIncomingHandler);"] # [doc = " ```"] # [doc = ""] # [doc = " <!--"] # [doc = " The marker above hides the generated documentation by wit-bindgen"] # [doc = " for this macro."] # [doc = " -->"] # [doc (inline)] pub use crate :: proxy :: _export_proxy as export ; } }
};
}
