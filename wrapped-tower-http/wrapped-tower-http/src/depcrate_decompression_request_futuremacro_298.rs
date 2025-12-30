// Generated macro for macro_298 (macro)
macro_rules! Depcrate_decompression_request_futuremacro_298 {
() => {
// Module: crate::decompression::request::future
// Provides: {"macro_298"}
// Dependencies: {}
pin_project ! { # [derive (Debug)] # [doc = " Response future of [`RequestDecompression`]"] pub struct RequestDecompressionFuture < F , B , E > where F : Future < Output = Result < Response < B >, E >>, B : Body { # [pin] kind : Kind < F , B , E >, } }
};
}
