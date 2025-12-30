// Generated macro for impl_11 (impl)
macro_rules! Depcrate_batch_requestimpl_11 {
() => {
// Module: crate::batch_request
// Provides: {"impl_11"}
// Dependencies: {}
impl Reply for GraphQLBatchResponse { fn into_response (self) -> WarpResponse { let mut resp = warp :: reply :: with_header (warp :: reply :: json (& self . 0) , "content-type" , "application/json" ,) . into_response () ; if self . 0 . is_ok () { if let Some (cache_control) = self . 0 . cache_control () . value () { if let Ok (value) = cache_control . try_into () { resp . headers_mut () . insert ("cache-control" , value) ; } } } resp . headers_mut () . extend (self . 0 . http_headers () . iter () . filter_map (| (name , value) | { HeaderName :: from_str (name . as_str ()) . ok () . zip (HeaderValue :: from_bytes (value . as_bytes ()) . ok ()) })) ; resp } }
};
}
