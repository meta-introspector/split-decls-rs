// Generated macro for impl_297 (impl)
macro_rules! Depcrate_scriptsimpl_297 {
() => {
// Module: crate::scripts
// Provides: {"impl_297"}
// Dependencies: {}
impl std :: str :: FromStr for Script { type Err = Error ; fn from_str (line : & str) -> Result < Script , Error > { let (codepoints , script) = parse_codepoint_association (line) ? ; Ok (Script { codepoints , script : script . to_string () }) } }
};
}
