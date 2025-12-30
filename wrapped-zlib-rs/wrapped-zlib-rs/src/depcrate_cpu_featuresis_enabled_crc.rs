// Generated macro for is_enabled_crc (function)
macro_rules! Depcrate_cpu_featuresis_enabled_crc {
() => {
// Module: crate::cpu_features
// Provides: {"is_enabled_crc"}
// Dependencies: {}
# [inline (always)] pub fn is_enabled_crc () -> bool { # [cfg (target_arch = "aarch64")] # [cfg (feature = "std")] return std :: arch :: is_aarch64_feature_detected ! ("crc") ; false }
};
}
