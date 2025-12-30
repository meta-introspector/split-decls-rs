// Generated macro for macro_1238 (macro)
macro_rules! Depcrate_services_fsmacro_1238 {
() => {
// Module: crate::services::fs
// Provides: {"macro_1238"}
// Dependencies: {}
pin_project ! { # [doc = " Adapter that turns an [`impl AsyncRead`][tokio::io::AsyncRead] to an [`impl Body`][http_body::Body]."] # [derive (Debug)] pub struct AsyncReadBody < T > { # [pin] reader : ReaderStream < T >, } }
};
}
