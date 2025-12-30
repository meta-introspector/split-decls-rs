// Generated macro for impl_13 (impl)
macro_rules! Depcrate_extracted_derived_bidi_classimpl_13 {
() => {
// Module: crate::extracted::derived_bidi_class
// Provides: {"impl_13"}
// Dependencies: {}
impl std :: str :: FromStr for DerivedBidiClass { type Err = Error ; fn from_str (line : & str) -> Result < DerivedBidiClass , Error > { let (codepoints , bidi_class) = parse_codepoint_association (line) ? ; Ok (DerivedBidiClass { codepoints , bidi_class : bidi_class . to_string () }) } }
};
}
