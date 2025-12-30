// Generated macro for impl_197 (impl)
macro_rules! Depcrate_compression_bodyimpl_197 {
() => {
// Module: crate::compression::body
// Provides: {"impl_197"}
// Dependencies: {}
impl < B : Body > BodyInner < B > { # [cfg (feature = "compression-gzip")] pub (crate) fn gzip (inner : WrapBody < GzipEncoder < B > >) -> Self { Self :: Gzip { inner } } # [cfg (feature = "compression-deflate")] pub (crate) fn deflate (inner : WrapBody < ZlibEncoder < B > >) -> Self { Self :: Deflate { inner } } # [cfg (feature = "compression-br")] pub (crate) fn brotli (inner : WrapBody < BrotliEncoder < B > >) -> Self { Self :: Brotli { inner } } # [cfg (feature = "compression-zstd")] pub (crate) fn zstd (inner : WrapBody < ZstdEncoder < B > >) -> Self { Self :: Zstd { inner } } pub (crate) fn identity (inner : B) -> Self { Self :: Identity { inner } } }
};
}
