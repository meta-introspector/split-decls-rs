// Generated macro for is_enabled_pclmulqdq (function)
macro_rules! Depcrate_cpu_featuresis_enabled_pclmulqdq {
() => {
// Module: crate::cpu_features
// Provides: {"is_enabled_pclmulqdq"}
// Dependencies: {}
# [inline (always)] pub fn is_enabled_pclmulqdq () -> bool { # [cfg (target_arch = "x86_64")] # [cfg (feature = "std")] return std :: is_x86_feature_detected ! ("pclmulqdq") && std :: is_x86_feature_detected ! ("sse4.1") ; false }
};
}
