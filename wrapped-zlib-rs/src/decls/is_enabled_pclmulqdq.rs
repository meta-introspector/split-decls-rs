macro_rules! is_enabled_pclmulqdq {
    () => {
        # [inline (always)] pub fn is_enabled_pclmulqdq () -> bool { # [cfg (target_arch = "x86_64")] # [cfg (feature = "std")] return std :: is_x86_feature_detected ! ("pclmulqdq") && std :: is_x86_feature_detected ! ("sse4.1") ; false }
    };
}

is_enabled_pclmulqdq!();