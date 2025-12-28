macro_rules! is_enabled_simd128 {
    () => {
        # [inline (always)] pub fn is_enabled_simd128 () -> bool { # [cfg (target_arch = "wasm32")] return cfg ! (target_feature = "simd128") ; false }
    };
}

is_enabled_simd128!()