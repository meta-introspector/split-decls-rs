// Generated macro for header (function)
macro_rules! Depcrate_filters_headerheader {
() => {
// Module: crate::filters::header
// Provides: {"header"}
// Dependencies: {}
# [doc = " Create a `Filter` that tries to parse the specified header."] # [doc = ""] # [doc = " This `Filter` will look for a header with supplied name, and try to"] # [doc = " parse to a `T`, otherwise rejects the request."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use std::net::SocketAddr;"] # [doc = ""] # [doc = " // Parse `content-length: 100` as a `u64`"] # [doc = " let content_length = warp::header::<u64>(\"content-length\");"] # [doc = ""] # [doc = " // Parse `host: 127.0.0.1:8080` as a `SocketAddr"] # [doc = " let local_host = warp::header::<SocketAddr>(\"host\");"] # [doc = ""] # [doc = " // Parse `foo: bar` into a `String`"] # [doc = " let foo = warp::header::<String>(\"foo\");"] # [doc = " ```"] pub fn header < T : FromStr + Send + 'static > (name : & 'static str ,) -> impl Filter < Extract = One < T > , Error = Rejection > + Copy { filter_fn_one (move | route | { tracing :: trace ! ("header({:?})" , name) ; let route = route . headers () . get (name) . ok_or_else (| | reject :: missing_header (name)) . and_then (| value | value . to_str () . map_err (| _ | reject :: invalid_header (name))) . and_then (| s | T :: from_str (s) . map_err (| _ | reject :: invalid_header (name))) ; future :: ready (route) }) }
};
}
