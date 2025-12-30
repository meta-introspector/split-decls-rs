// Generated macro for impl_55 (impl)
macro_rules! Depcrate_compressionimpl_55 {
() => {
// Module: crate::compression
// Provides: {"impl_55"}
// Dependencies: {}
impl Default for CompressionMethod { fn default () -> Self { # [cfg (feature = "_deflate-any")] return CompressionMethod :: Deflated ; # [cfg (not (feature = "_deflate-any"))] return CompressionMethod :: Stored ; } }
};
}
