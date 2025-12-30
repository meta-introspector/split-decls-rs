// Generated macro for compress_bound_help (function)
macro_rules! Depcrate_deflatecompress_bound_help {
() => {
// Module: crate::deflate
// Provides: {"compress_bound_help"}
// Dependencies: {}
const fn compress_bound_help (source_len : usize , wrap_len : usize) -> usize { source_len . wrapping_add (if source_len == 0 { 1 } else { 0 }) . wrapping_add (if source_len < 9 { 1 } else { 0 }) . wrapping_add (deflate_quick_overhead (source_len)) . wrapping_add (DEFLATE_BLOCK_OVERHEAD) . wrapping_add (wrap_len) }
};
}
