// Generated macro for impl_445 (impl)
macro_rules! Depcrate_filters_logimpl_445 {
() => {
// Module: crate::filters::log
// Provides: {"impl_445"}
// Dependencies: {}
impl < 'a > Info < 'a > { # [doc = " View the `http::Method` of the request."] pub fn method (& self) -> & http :: Method { self . route . method () } # [doc = " View the URI path of the request."] pub fn path (& self) -> & str { self . route . full_path () } # [doc = " View the `http::Version` of the request."] pub fn version (& self) -> http :: Version { self . route . version () } # [doc = " View the `http::StatusCode` of the response."] pub fn status (& self) -> http :: StatusCode { self . status } # [doc = " View the referer of the request."] pub fn referer (& self) -> Option < & str > { self . route . headers () . get (header :: REFERER) . and_then (| v | v . to_str () . ok ()) } # [doc = " View the user agent of the request."] pub fn user_agent (& self) -> Option < & str > { self . route . headers () . get (header :: USER_AGENT) . and_then (| v | v . to_str () . ok ()) } # [doc = " View the `Duration` that elapsed for the request."] pub fn elapsed (& self) -> Duration { tokio :: time :: Instant :: now () . into_std () - self . start } # [doc = " View the host of the request"] pub fn host (& self) -> Option < & str > { self . route . headers () . get (header :: HOST) . and_then (| v | v . to_str () . ok ()) } # [doc = " Access the full headers of the request"] pub fn request_headers (& self) -> & http :: HeaderMap { self . route . headers () } }
};
}
