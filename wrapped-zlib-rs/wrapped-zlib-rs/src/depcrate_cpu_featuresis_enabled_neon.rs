// Generated macro for is_enabled_neon (function)
macro_rules! Depcrate_cpu_featuresis_enabled_neon {
() => {
// Module: crate::cpu_features
// Provides: {"is_enabled_neon"}
// Dependencies: {}
# [inline (always)] pub fn is_enabled_neon () -> bool { # [cfg (target_arch = "aarch64")] { # [cfg (target_feature = "neon")] return true ; # [cfg (feature = "std")] return std :: arch :: is_aarch64_feature_detected ! ("neon") ; } false }
};
}
