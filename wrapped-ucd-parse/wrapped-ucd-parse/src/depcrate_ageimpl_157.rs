// Generated macro for impl_157 (impl)
macro_rules! Depcrate_ageimpl_157 {
() => {
// Module: crate::age
// Provides: {"impl_157"}
// Dependencies: {}
impl std :: str :: FromStr for Age { type Err = Error ; fn from_str (line : & str) -> Result < Age , Error > { let (codepoints , script) = parse_codepoint_association (line) ? ; Ok (Age { codepoints , age : script . to_string () }) } }
};
}
