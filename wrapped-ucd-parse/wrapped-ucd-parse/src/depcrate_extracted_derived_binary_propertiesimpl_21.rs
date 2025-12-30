// Generated macro for impl_21 (impl)
macro_rules! Depcrate_extracted_derived_binary_propertiesimpl_21 {
() => {
// Module: crate::extracted::derived_binary_properties
// Provides: {"impl_21"}
// Dependencies: {}
impl std :: str :: FromStr for DerivedBinaryProperties { type Err = Error ; fn from_str (line : & str) -> Result < DerivedBinaryProperties , Error > { let (codepoints , property) = parse_codepoint_association (line) ? ; Ok (DerivedBinaryProperties { codepoints , property : property . to_string () , }) } }
};
}
