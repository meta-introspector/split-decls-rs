// Generated macro for impl_307 (impl)
macro_rules! Depcrate_decompression_request_layerimpl_307 {
() => {
// Module: crate::decompression::request::layer
// Provides: {"impl_307"}
// Dependencies: {}
impl < S > Layer < S > for RequestDecompressionLayer { type Service = RequestDecompression < S > ; fn layer (& self , service : S) -> Self :: Service { RequestDecompression { inner : service , accept : self . accept , pass_through_unaccepted : self . pass_through_unaccepted , } } }
};
}
