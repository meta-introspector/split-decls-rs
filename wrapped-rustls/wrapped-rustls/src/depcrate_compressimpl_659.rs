// Generated macro for impl_659 (impl)
macro_rules! Depcrate_compressimpl_659 {
() => {
// Module: crate::compress
// Provides: {"impl_659"}
// Dependencies: {}
impl Default for CompressionCache { fn default () -> Self { # [cfg (feature = "std")] { Self :: new (4) } # [cfg (not (feature = "std"))] { Self :: Disabled } } }
};
}
