// Generated macro for PropagateHeader (struct)
macro_rules! Depcrate_propagate_headerPropagateHeader {
() => {
// Module: crate::propagate_header
// Provides: {"PropagateHeader"}
// Dependencies: {}
# [doc = " Middleware that propagates headers from requests to responses."] # [doc = ""] # [doc = " If the header is present on the request it'll be applied to the response as well. This could"] # [doc = " for example be used to propagate headers such as `X-Request-Id`."] # [doc = ""] # [doc = " See the [module docs](crate::propagate_header) for more details."] # [derive (Clone , Debug)] pub struct PropagateHeader < S > { inner : S , header : HeaderName , }
};
}
