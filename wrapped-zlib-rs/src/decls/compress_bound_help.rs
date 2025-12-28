macro_rules! compress_bound_help {
    () => {
        const fn compress_bound_help (source_len : usize , wrap_len : usize) -> usize { source_len . wrapping_add (if source_len == 0 { 1 } else { 0 }) . wrapping_add (if source_len < 9 { 1 } else { 0 }) . wrapping_add (deflate_quick_overhead (source_len)) . wrapping_add (DEFLATE_BLOCK_OVERHEAD) . wrapping_add (wrap_len) }
    };
}

compress_bound_help!();