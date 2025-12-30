// Generated macro for SetRequestHeaderLayer (struct)
macro_rules! Depcrate_set_header_requestSetRequestHeaderLayer {
() => {
// Module: crate::set_header::request
// Provides: {"SetRequestHeaderLayer"}
// Dependencies: {}
# [doc = " Layer that applies [`SetRequestHeader`] which adds a request header."] # [doc = ""] # [doc = " See [`SetRequestHeader`] for more details."] pub struct SetRequestHeaderLayer < M > { header_name : HeaderName , make : M , mode : InsertHeaderMode , }
};
}
