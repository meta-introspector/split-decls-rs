// Generated macro for impl_289 (impl)
macro_rules! Depcrate_script_extensionsimpl_289 {
() => {
// Module: crate::script_extensions
// Provides: {"impl_289"}
// Dependencies: {}
impl std :: str :: FromStr for ScriptExtension { type Err = Error ; fn from_str (line : & str) -> Result < ScriptExtension , Error > { let (codepoints , scripts) = parse_codepoint_association (line) ? ; Ok (ScriptExtension { codepoints , scripts : scripts . split_whitespace () . map (str :: to_string) . collect () , }) } }
};
}
