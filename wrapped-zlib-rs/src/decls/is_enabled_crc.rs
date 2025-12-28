macro_rules! is_enabled_crc {
    () => {
        # [inline (always)] pub fn is_enabled_crc () -> bool { # [cfg (target_arch = "aarch64")] # [cfg (feature = "std")] return std :: arch :: is_aarch64_feature_detected ! ("crc") ; false }
    };
}

is_enabled_crc!();