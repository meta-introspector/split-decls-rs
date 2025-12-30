// Generated macro for impl_610 (impl)
macro_rules! Depcrate_filters_traceimpl_610 {
() => {
// Module: crate::filters::trace
// Provides: {"impl_610"}
// Dependencies: {}
impl < 'a > Info < 'a > { # [doc = " View the `http::Method` of the request."] pub fn method (& self) -> & http :: Method { self . route . method () } # [doc = " View the URI path of the request."] pub fn path (& self) -> & str { self . route . full_path () } # [doc = " View the `http::Version` of the request."] pub fn version (& self) -> http :: Version { self . route . version () } # [doc = " View the referer of the request."] pub fn referer (& self) -> Option < & str > { self . route . headers () . get (header :: REFERER) . and_then (| v | v . to_str () . ok ()) } # [doc = " View the user agent of the request."] pub fn user_agent (& self) -> Option < & str > { self . route . headers () . get (header :: USER_AGENT) . and_then (| v | v . to_str () . ok ()) } # [doc = " View the host of the request"] pub fn host (& self) -> Option < & str > { self . route . headers () . get (header :: HOST) . and_then (| v | v . to_str () . ok ()) } # [doc = " View the request headers."] pub fn request_headers (& self) -> & http :: HeaderMap { self . route . headers () } }
};
}
