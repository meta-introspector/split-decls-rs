// Generated macro for impl_86 (impl)
macro_rules! Depcrate_http_compat_conversionsimpl_86 {
() => {
// Module: crate::http_compat::conversions
// Provides: {"impl_86"}
// Dependencies: {}
impl TryFrom < Headers > for http :: HeaderMap { type Error = ErrorCode ; fn try_from (headers : Headers) -> Result < Self , Self :: Error > { headers . copy_all () . into_iter () . try_fold (http :: HeaderMap :: new () , | mut map , (k , v) | { let v = http :: HeaderValue :: from_bytes (& v) . map_err (to_internal_error_code) ? ; let k : http :: HeaderName = k . parse () . map_err (to_internal_error_code) ? ; map . append (k , v) ; Ok (map) }) } }
};
}
