// Generated macro for impl_328 (impl)
macro_rules! Depcrate_unicode_dataimpl_328 {
() => {
// Module: crate::unicode_data
// Provides: {"impl_328"}
// Dependencies: {}
impl UnicodeDataDecomposition { # [doc = " Create a new decomposition mapping with the given tag and codepoints."] # [doc = ""] # [doc = " If there are too many codepoints, then an error is returned."] pub fn new (tag : Option < UnicodeDataDecompositionTag > , mapping : & [Codepoint] ,) -> Result < UnicodeDataDecomposition , Error > { let mut x = UnicodeDataDecomposition :: default () ; x . tag = tag ; for & cp in mapping { x . push (cp) ? ; } Ok (x) } # [doc = " Add a new codepoint to this decomposition's mapping."] # [doc = ""] # [doc = " If the mapping is already full, then this returns an error."] pub fn push (& mut self , cp : Codepoint) -> Result < () , Error > { if self . len >= self . mapping . len () { return err ! ("invalid decomposition mapping (too many codepoints)") ; } self . mapping [self . len] = cp ; self . len += 1 ; Ok (()) } # [doc = " Return the mapping as a slice of codepoints. The slice returned"] # [doc = " has length equivalent to the number of codepoints in this mapping."] pub fn mapping (& self) -> & [Codepoint] { & self . mapping [.. self . len] } # [doc = " Returns true if and only if this decomposition mapping is canonical."] pub fn is_canonical (& self) -> bool { self . tag . is_none () } }
};
}
