// Generated macro for impl_308 (impl)
macro_rules! Depcrate_filters_compressionimpl_308 {
() => {
// Module: crate::filters::compression
// Provides: {"impl_308"}
// Dependencies: {}
impl < FN , F > WrapSealed < F > for Compression < FN > where FN : Fn (CompressionProps) -> Response + Clone + Send , F : Filter + Clone + Send , F :: Extract : Reply , F :: Error : IsReject , { type Wrapped = WithCompression < FN , F > ; fn wrap (& self , filter : F) -> Self :: Wrapped { WithCompression { filter , compress : self . clone () , } } }
};
}
