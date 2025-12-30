// Generated macro for impl_93 (impl)
macro_rules! Depcrate_extracted_derived_numeric_typeimpl_93 {
() => {
// Module: crate::extracted::derived_numeric_type
// Provides: {"impl_93"}
// Dependencies: {}
impl std :: str :: FromStr for DerivedNumericType { type Err = Error ; fn from_str (line : & str) -> Result < DerivedNumericType , Error > { let (codepoints , numeric_type) = parse_codepoint_association (line) ? ; Ok (DerivedNumericType { codepoints , numeric_type : numeric_type . to_string () , }) } }
};
}
