// Generated macro for Response (type)
macro_rules! Depcrate_http_compatResponse {
() => {
// Module: crate::http_compat
// Provides: {"Response"}
// Dependencies: {}
# [doc = " A type alias for an HTTP response with a customizable body type."] # [doc = ""] # [doc = " This is a convenience wrapper around [`http::Response`], parameterized"] # [doc = " by the body type `T`. By default, it uses [`IncomingResponseBody`],"] # [doc = " which represents the standard incoming body type used by this runtime."] # [doc = ""] # [doc = " # See also"] # [doc = " - [`IncomingResponseBody`]: The body type for inbound HTTP responses."] # [doc = " - [`http::Response`]: The standard HTTP response type from the `http` crate."] pub type Response < T = IncomingResponseBody > = http :: Response < T > ;
};
}
