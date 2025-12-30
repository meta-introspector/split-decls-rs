// Generated macro for SetRequestIdLayer (struct)
macro_rules! Depcrate_request_idSetRequestIdLayer {
() => {
// Module: crate::request_id
// Provides: {"SetRequestIdLayer"}
// Dependencies: {}
# [doc = " Set request id headers and extensions on requests."] # [doc = ""] # [doc = " This layer applies the [`SetRequestId`] middleware."] # [doc = ""] # [doc = " See the [module docs](self) and [`SetRequestId`] for more details."] # [derive (Debug , Clone)] pub struct SetRequestIdLayer < M > { header_name : HeaderName , make_request_id : M , }
};
}
