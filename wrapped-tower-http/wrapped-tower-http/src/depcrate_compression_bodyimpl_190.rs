// Generated macro for impl_190 (impl)
macro_rules! Depcrate_compression_bodyimpl_190 {
() => {
// Module: crate::compression::body
// Provides: {"impl_190"}
// Dependencies: {}
impl < B > Default for CompressionBody < B > where B : Body + Default , { fn default () -> Self { Self { inner : BodyInner :: Identity { inner : B :: default () , } , } } }
};
}
