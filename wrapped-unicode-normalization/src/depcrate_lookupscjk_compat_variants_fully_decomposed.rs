// Generated macro for cjk_compat_variants_fully_decomposed (function)
macro_rules! Depcrate_lookupscjk_compat_variants_fully_decomposed {
() => {
// Module: crate::lookups
// Provides: {"cjk_compat_variants_fully_decomposed"}
// Dependencies: {}
pub (crate) fn cjk_compat_variants_fully_decomposed (c : char) -> Option < & 'static [char] > { mph_lookup (c . into () , CJK_COMPAT_VARIANTS_DECOMPOSED_SALT , CJK_COMPAT_VARIANTS_DECOMPOSED_KV , pair_lookup_fk , pair_lookup_fv_opt , None ,) . map (| (start , len) | & CJK_COMPAT_VARIANTS_DECOMPOSED_CHARS [start as usize ..] [.. len as usize]) }
};
}
