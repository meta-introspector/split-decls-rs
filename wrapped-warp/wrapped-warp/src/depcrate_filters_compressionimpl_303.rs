// Generated macro for impl_303 (impl)
macro_rules! Depcrate_filters_compressionimpl_303 {
() => {
// Module: crate::filters::compression
// Provides: {"impl_303"}
// Dependencies: {}
impl From < CompressionAlgo > for HeaderValue { # [inline] fn from (algo : CompressionAlgo) -> Self { HeaderValue :: from_static (match algo { # [cfg (feature = "compression-brotli")] CompressionAlgo :: BR => "br" , # [cfg (feature = "compression-gzip")] CompressionAlgo :: DEFLATE => "deflate" , # [cfg (feature = "compression-gzip")] CompressionAlgo :: GZIP => "gzip" , }) } }
};
}
