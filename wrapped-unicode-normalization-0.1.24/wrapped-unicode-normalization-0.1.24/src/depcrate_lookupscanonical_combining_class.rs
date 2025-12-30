// Generated macro for canonical_combining_class (function)
macro_rules! Depcrate_lookupscanonical_combining_class {
() => {
// Module: crate::lookups
// Provides: {"canonical_combining_class"}
// Dependencies: {}
# [doc = " Look up the canonical combining class for a codepoint."] # [doc = ""] # [doc = " The value returned is as defined in the Unicode Character Database."] pub fn canonical_combining_class (c : char) -> u8 { mph_lookup (c . into () , CANONICAL_COMBINING_CLASS_SALT , CANONICAL_COMBINING_CLASS_KV , u8_lookup_fk , u8_lookup_fv , 0 ,) }
};
}
