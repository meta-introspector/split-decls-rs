// Generated macro for impl_340 (impl)
macro_rules! Depcrate_decompression_bodyimpl_340 {
() => {
// Module: crate::decompression::body
// Provides: {"impl_340"}
// Dependencies: {}
impl < B > Default for DecompressionBody < B > where B : Body + Default , { fn default () -> Self { Self { inner : BodyInner :: Identity { inner : B :: default () , } , } } }
};
}
