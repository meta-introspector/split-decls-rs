// Generated macro for impl_32 (impl)
macro_rules! Depcrate_requestimpl_32 {
() => {
// Module: crate::request
// Provides: {"impl_32"}
// Dependencies: {}
impl Reply for GraphQLResponse { fn into_response (self) -> WarpResponse { GraphQLBatchResponse (self . 0 . into ()) . into_response () } }
};
}
