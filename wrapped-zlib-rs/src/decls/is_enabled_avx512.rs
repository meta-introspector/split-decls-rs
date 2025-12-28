macro_rules! is_enabled_avx512 {
    () => {
        # [inline (always)] pub fn is_enabled_avx512 () -> bool { # [cfg (any (target_arch = "x86_64" , target_arch = "x86"))] # [cfg (feature = "std")] return std :: is_x86_feature_detected ! ("avx512f") ; false }
    };
}

is_enabled_avx512!();