// Generated macro for impl_373 (impl)
macro_rules! Depcrate_decompression_layerimpl_373 {
() => {
// Module: crate::decompression::layer
// Provides: {"impl_373"}
// Dependencies: {}
impl < S > Layer < S > for DecompressionLayer { type Service = Decompression < S > ; fn layer (& self , service : S) -> Self :: Service { Decompression { inner : service , accept : self . accept , } } }
};
}
