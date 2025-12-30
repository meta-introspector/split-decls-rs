// Generated macro for SetRequestId (struct)
macro_rules! Depcrate_request_idSetRequestId {
() => {
// Module: crate::request_id
// Provides: {"SetRequestId"}
// Dependencies: {}
# [doc = " Set request id headers and extensions on requests."] # [doc = ""] # [doc = " See the [module docs](self) for an example."] # [doc = ""] # [doc = " If [`MakeRequestId::make_request_id`] returns `Some(_)` and the request doesn't already have a"] # [doc = " header with the same name, then the header will be inserted."] # [doc = ""] # [doc = " Additionally [`RequestId`] will be inserted into [`Request::extensions`] so other"] # [doc = " services can access it."] # [derive (Debug , Clone)] pub struct SetRequestId < S , M > { inner : S , header_name : HeaderName , make_request_id : M , }
};
}
