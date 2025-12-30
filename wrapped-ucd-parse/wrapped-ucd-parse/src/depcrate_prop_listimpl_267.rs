// Generated macro for impl_267 (impl)
macro_rules! Depcrate_prop_listimpl_267 {
() => {
// Module: crate::prop_list
// Provides: {"impl_267"}
// Dependencies: {}
impl std :: str :: FromStr for Property { type Err = Error ; fn from_str (line : & str) -> Result < Property , Error > { let (codepoints , property) = parse_codepoint_association (line) ? ; Ok (Property { codepoints , property : property . to_string () }) } }
};
}
