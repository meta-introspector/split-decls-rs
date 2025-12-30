// Generated macro for impl_29 (impl)
macro_rules! Depcrate_extracted_derived_combining_classimpl_29 {
() => {
// Module: crate::extracted::derived_combining_class
// Provides: {"impl_29"}
// Dependencies: {}
impl std :: str :: FromStr for DerivedCombiningClass { type Err = Error ; fn from_str (line : & str) -> Result < DerivedCombiningClass , Error > { let (codepoints , combining_class) = parse_codepoint_association (line) ? ; Ok (DerivedCombiningClass { codepoints , combining_class : combining_class . to_string () , }) } }
};
}
