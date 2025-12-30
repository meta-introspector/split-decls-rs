// Generated macro for impl_198 (impl)
macro_rules! Depcrate_core_propertiesimpl_198 {
() => {
// Module: crate::core_properties
// Provides: {"impl_198"}
// Dependencies: {}
impl std :: str :: FromStr for CoreProperty { type Err = Error ; fn from_str (line : & str) -> Result < CoreProperty , Error > { let (codepoints , property) = parse_codepoint_association (line) ? ; Ok (CoreProperty { codepoints , property : property . to_string () }) } }
};
}
