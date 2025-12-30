// Generated macro for canonical_fully_decomposed (function)
macro_rules! Depcrate_lookupscanonical_fully_decomposed {
() => {
// Module: crate::lookups
// Provides: {"canonical_fully_decomposed"}
// Dependencies: {}
pub (crate) fn canonical_fully_decomposed (c : char) -> Option < & 'static [char] > { mph_lookup (c . into () , CANONICAL_DECOMPOSED_SALT , CANONICAL_DECOMPOSED_KV , pair_lookup_fk , pair_lookup_fv_opt , None ,) . map (| (start , len) | & CANONICAL_DECOMPOSED_CHARS [start as usize ..] [.. len as usize]) }
};
}
