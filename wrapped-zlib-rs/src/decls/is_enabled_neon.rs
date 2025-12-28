macro_rules! is_enabled_neon {
    () => {
        # [inline (always)] pub fn is_enabled_neon () -> bool { # [cfg (target_arch = "aarch64")] { # [cfg (target_feature = "neon")] return true ; # [cfg (feature = "std")] return std :: arch :: is_aarch64_feature_detected ! ("neon") ; } false }
    };
}

is_enabled_neon!();