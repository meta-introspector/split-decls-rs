// Generated macro for Request (type)
macro_rules! Depcrate_http_compatRequest {
() => {
// Module: crate::http_compat
// Provides: {"Request"}
// Dependencies: {}
# [doc = " A type alias for an HTTP request with a customizable body type."] # [doc = ""] # [doc = " This is a convenience wrapper around [`http::Request`], parameterized"] # [doc = " by the body type `T`. By default, it uses [`IncomingRequestBody`],"] # [doc = " which represents the standard incoming body used by this runtime."] # [doc = ""] # [doc = " # See also"] # [doc = " - [`IncomingRequestBody`]: The body type for inbound HTTP requests."] # [doc = " - [`http::Request`]: The standard HTTP request type from the `http` crate."] pub type Request < T = IncomingRequestBody > = http :: Request < T > ;
};
}
