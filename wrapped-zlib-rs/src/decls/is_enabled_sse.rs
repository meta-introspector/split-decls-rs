macro_rules! is_enabled_sse {
    () => {
        # [inline (always)] pub fn is_enabled_sse () -> bool { # [cfg (any (target_arch = "x86_64" , target_arch = "x86"))] # [cfg (feature = "std")] return std :: is_x86_feature_detected ! ("sse") ; false }
    };
}

is_enabled_sse!();