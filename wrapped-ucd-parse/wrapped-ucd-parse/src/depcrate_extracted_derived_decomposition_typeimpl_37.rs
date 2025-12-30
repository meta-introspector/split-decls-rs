// Generated macro for impl_37 (impl)
macro_rules! Depcrate_extracted_derived_decomposition_typeimpl_37 {
() => {
// Module: crate::extracted::derived_decomposition_type
// Provides: {"impl_37"}
// Dependencies: {}
impl std :: str :: FromStr for DerivedDecompositionType { type Err = Error ; fn from_str (line : & str) -> Result < DerivedDecompositionType , Error > { let (codepoints , decomposition_type) = parse_codepoint_association (line) ? ; Ok (DerivedDecompositionType { codepoints , decomposition_type : decomposition_type . to_string () , }) } }
};
}
