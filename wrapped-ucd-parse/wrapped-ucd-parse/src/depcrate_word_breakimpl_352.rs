// Generated macro for impl_352 (impl)
macro_rules! Depcrate_word_breakimpl_352 {
() => {
// Module: crate::word_break
// Provides: {"impl_352"}
// Dependencies: {}
impl std :: str :: FromStr for WordBreakTest { type Err = Error ; fn from_str (line : & str) -> Result < WordBreakTest , Error > { let (groups , comment) = parse_break_test (line) ? ; Ok (WordBreakTest { words : groups , comment }) } }
};
}
