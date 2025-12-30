// Generated macro for macro_875 (macro)
macro_rules! Depcrate_corsmacro_875 {
() => {
// Module: crate::cors
// Provides: {"macro_875"}
// Dependencies: {}
pin_project ! { # [project = KindProj] enum Kind < F > { CorsCall { # [pin] allow_origin_future : AllowOriginFuture , allow_origin_complete : bool , # [pin] future : F , headers : HeaderMap , } , PreflightCall { # [pin] allow_origin_future : AllowOriginFuture , headers : HeaderMap , } , } }
};
}
