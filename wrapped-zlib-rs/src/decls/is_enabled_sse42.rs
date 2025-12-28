macro_rules! is_enabled_sse42 {
    () => {
        # [inline (always)] pub fn is_enabled_sse42 () -> bool { # [cfg (any (target_arch = "x86_64" , target_arch = "x86"))] # [cfg (feature = "std")] return std :: is_x86_feature_detected ! ("sse4.2") ; false }
    };
}

is_enabled_sse42!()