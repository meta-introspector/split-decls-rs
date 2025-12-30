// Generated macro for impl_352 (impl)
macro_rules! Depcrate_decompression_bodyimpl_352 {
() => {
// Module: crate::decompression::body
// Provides: {"impl_352"}
// Dependencies: {}
impl < B : Body > BodyInner < B > { # [cfg (feature = "decompression-gzip")] pub (crate) fn gzip (inner : WrapBody < GzipDecoder < B > >) -> Self { Self :: Gzip { inner } } # [cfg (feature = "decompression-deflate")] pub (crate) fn deflate (inner : WrapBody < ZlibDecoder < B > >) -> Self { Self :: Deflate { inner } } # [cfg (feature = "decompression-br")] pub (crate) fn brotli (inner : WrapBody < BrotliDecoder < B > >) -> Self { Self :: Brotli { inner } } # [cfg (feature = "decompression-zstd")] pub (crate) fn zstd (inner : WrapBody < ZstdDecoder < B > >) -> Self { Self :: Zstd { inner } } pub (crate) fn identity (inner : B) -> Self { Self :: Identity { inner } } }
};
}
