// Generated macro for is_enabled_avx512 (function)
macro_rules! Depcrate_cpu_featuresis_enabled_avx512 {
() => {
// Module: crate::cpu_features
// Provides: {"is_enabled_avx512"}
// Dependencies: {}
# [inline (always)] pub fn is_enabled_avx512 () -> bool { # [cfg (any (target_arch = "x86_64" , target_arch = "x86"))] # [cfg (feature = "std")] return std :: is_x86_feature_detected ! ("avx512f") ; false }
};
}
