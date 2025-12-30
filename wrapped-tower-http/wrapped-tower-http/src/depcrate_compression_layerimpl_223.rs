// Generated macro for impl_223 (impl)
macro_rules! Depcrate_compression_layerimpl_223 {
() => {
// Module: crate::compression::layer
// Provides: {"impl_223"}
// Dependencies: {}
impl < S , P > Layer < S > for CompressionLayer < P > where P : Predicate , { type Service = Compression < S , P > ; fn layer (& self , inner : S) -> Self :: Service { Compression { inner , accept : self . accept , predicate : self . predicate . clone () , quality : self . quality , } } }
};
}
