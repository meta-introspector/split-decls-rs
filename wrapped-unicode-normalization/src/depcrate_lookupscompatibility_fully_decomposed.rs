// Generated macro for compatibility_fully_decomposed (function)
macro_rules! Depcrate_lookupscompatibility_fully_decomposed {
() => {
// Module: crate::lookups
// Provides: {"compatibility_fully_decomposed"}
// Dependencies: {}
pub (crate) fn compatibility_fully_decomposed (c : char) -> Option < & 'static [char] > { mph_lookup (c . into () , COMPATIBILITY_DECOMPOSED_SALT , COMPATIBILITY_DECOMPOSED_KV , pair_lookup_fk , pair_lookup_fv_opt , None ,) . map (| (start , len) | & COMPATIBILITY_DECOMPOSED_CHARS [start as usize ..] [.. len as usize]) }
};
}
