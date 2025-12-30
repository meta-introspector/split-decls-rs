// Generated macro for to_internal_error_code (function)
macro_rules! Depcrate_http_compatto_internal_error_code {
() => {
// Module: crate::http_compat
// Provides: {"to_internal_error_code"}
// Dependencies: {}
fn to_internal_error_code (e : impl :: std :: fmt :: Display) -> ErrorCode { ErrorCode :: InternalError (Some (e . to_string ())) }
};
}
