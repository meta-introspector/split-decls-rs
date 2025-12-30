// Generated macro for macro_299 (macro)
macro_rules! Depcrate_decompression_request_futuremacro_299 {
() => {
// Module: crate::decompression::request::future
// Provides: {"macro_299"}
// Dependencies: {}
pin_project ! { # [derive (Debug)] # [project = StateProj] enum Kind < F , B , E > where F : Future < Output = Result < Response < B >, E >>, B : Body { Inner { # [pin] fut : F } , Unsupported { # [pin] accept : AcceptEncoding } , } }
};
}
