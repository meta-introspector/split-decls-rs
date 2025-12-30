// Generated macro for DecompressionLayer (struct)
macro_rules! Depcrate_decompression_layerDecompressionLayer {
() => {
// Module: crate::decompression::layer
// Provides: {"DecompressionLayer"}
// Dependencies: {}
# [doc = " Decompresses response bodies of the underlying service."] # [doc = ""] # [doc = " This adds the `Accept-Encoding` header to requests and transparently decompresses response"] # [doc = " bodies based on the `Content-Encoding` header."] # [doc = ""] # [doc = " See the [module docs](crate::decompression) for more details."] # [derive (Debug , Default , Clone)] pub struct DecompressionLayer { accept : AcceptEncoding , }
};
}
