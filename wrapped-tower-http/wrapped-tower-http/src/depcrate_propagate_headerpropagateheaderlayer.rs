// Generated macro for PropagateHeaderLayer (struct)
macro_rules! Depcrate_propagate_headerPropagateHeaderLayer {
() => {
// Module: crate::propagate_header
// Provides: {"PropagateHeaderLayer"}
// Dependencies: {}
# [doc = " Layer that applies [`PropagateHeader`] which propagates headers from requests to responses."] # [doc = ""] # [doc = " If the header is present on the request it'll be applied to the response as well. This could"] # [doc = " for example be used to propagate headers such as `X-Request-Id`."] # [doc = ""] # [doc = " See the [module docs](crate::propagate_header) for more details."] # [derive (Clone , Debug)] pub struct PropagateHeaderLayer { header : HeaderName , }
};
}
