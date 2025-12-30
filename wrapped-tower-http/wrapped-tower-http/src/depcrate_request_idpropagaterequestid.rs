// Generated macro for PropagateRequestId (struct)
macro_rules! Depcrate_request_idPropagateRequestId {
() => {
// Module: crate::request_id
// Provides: {"PropagateRequestId"}
// Dependencies: {}
# [doc = " Propagate request ids from requests to responses."] # [doc = ""] # [doc = " See the [module docs](self) for an example."] # [doc = ""] # [doc = " If the request contains a matching header that header will be applied to responses. If a"] # [doc = " [`RequestId`] extension is also present it will be propagated as well."] # [derive (Debug , Clone)] pub struct PropagateRequestId < S > { inner : S , header_name : HeaderName , }
};
}
