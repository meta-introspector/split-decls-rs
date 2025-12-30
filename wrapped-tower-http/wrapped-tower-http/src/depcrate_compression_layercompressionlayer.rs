// Generated macro for CompressionLayer (struct)
macro_rules! Depcrate_compression_layerCompressionLayer {
() => {
// Module: crate::compression::layer
// Provides: {"CompressionLayer"}
// Dependencies: {}
# [doc = " Compress response bodies of the underlying service."] # [doc = ""] # [doc = " This uses the `Accept-Encoding` header to pick an appropriate encoding and adds the"] # [doc = " `Content-Encoding` header to responses."] # [doc = ""] # [doc = " See the [module docs](crate::compression) for more details."] # [derive (Clone , Debug , Default)] pub struct CompressionLayer < P = DefaultPredicate > { accept : AcceptEncoding , predicate : P , quality : CompressionLevel , }
};
}
