macro_rules! compress_bound {
    () => {
        pub const fn compress_bound (source_len : usize) -> usize { compress_bound_help (source_len , ZLIB_WRAPLEN) }
    };
}

compress_bound!();