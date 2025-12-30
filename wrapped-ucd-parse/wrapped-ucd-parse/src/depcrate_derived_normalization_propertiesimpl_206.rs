// Generated macro for impl_206 (impl)
macro_rules! Depcrate_derived_normalization_propertiesimpl_206 {
() => {
// Module: crate::derived_normalization_properties
// Provides: {"impl_206"}
// Dependencies: {}
impl std :: str :: FromStr for DerivedNormalizationProperty { type Err = Error ; fn from_str (line : & str) -> Result < DerivedNormalizationProperty , Error > { let (codepoints , property) = parse_codepoint_association (line) ? ; Ok (DerivedNormalizationProperty { codepoints , property : property . to_string () , }) } }
};
}
