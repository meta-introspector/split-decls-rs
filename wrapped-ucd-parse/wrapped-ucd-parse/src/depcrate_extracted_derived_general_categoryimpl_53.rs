// Generated macro for impl_53 (impl)
macro_rules! Depcrate_extracted_derived_general_categoryimpl_53 {
() => {
// Module: crate::extracted::derived_general_category
// Provides: {"impl_53"}
// Dependencies: {}
impl std :: str :: FromStr for DerivedGeneralCategory { type Err = Error ; fn from_str (line : & str) -> Result < DerivedGeneralCategory , Error > { let (codepoints , general_category) = parse_codepoint_association (line) ? ; Ok (DerivedGeneralCategory { codepoints , general_category : general_category . to_string () , }) } }
};
}
