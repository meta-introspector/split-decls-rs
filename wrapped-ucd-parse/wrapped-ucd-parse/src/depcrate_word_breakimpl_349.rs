// Generated macro for impl_349 (impl)
macro_rules! Depcrate_word_breakimpl_349 {
() => {
// Module: crate::word_break
// Provides: {"impl_349"}
// Dependencies: {}
impl std :: str :: FromStr for WordBreak { type Err = Error ; fn from_str (line : & str) -> Result < WordBreak , Error > { let (codepoints , value) = parse_codepoint_association (line) ? ; Ok (WordBreak { codepoints , value : value . to_string () }) } }
};
}
