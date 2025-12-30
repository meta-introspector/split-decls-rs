// Generated macro for is_enabled_sse (function)
macro_rules! Depcrate_cpu_featuresis_enabled_sse {
() => {
// Module: crate::cpu_features
// Provides: {"is_enabled_sse"}
// Dependencies: {}
# [inline (always)] pub fn is_enabled_sse () -> bool { # [cfg (any (target_arch = "x86_64" , target_arch = "x86"))] # [cfg (feature = "std")] return std :: is_x86_feature_detected ! ("sse") ; false }
};
}
