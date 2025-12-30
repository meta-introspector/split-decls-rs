// Generated macro for is_enabled_simd128 (function)
macro_rules! Depcrate_cpu_featuresis_enabled_simd128 {
() => {
// Module: crate::cpu_features
// Provides: {"is_enabled_simd128"}
// Dependencies: {}
# [inline (always)] pub fn is_enabled_simd128 () -> bool { # [cfg (target_arch = "wasm32")] return cfg ! (target_feature = "simd128") ; false }
};
}
