macro_rules! stream_safe_trailing_nonstarters {
    () => {
        pub fn stream_safe_trailing_nonstarters (c : char) -> usize { mph_lookup (c . into () , TRAILING_NONSTARTERS_SALT , TRAILING_NONSTARTERS_KV , u8_lookup_fk , u8_lookup_fv , 0 ,) as usize }
    };
}

stream_safe_trailing_nonstarters!();