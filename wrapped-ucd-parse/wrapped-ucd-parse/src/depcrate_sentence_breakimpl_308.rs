// Generated macro for impl_308 (impl)
macro_rules! Depcrate_sentence_breakimpl_308 {
() => {
// Module: crate::sentence_break
// Provides: {"impl_308"}
// Dependencies: {}
impl std :: str :: FromStr for SentenceBreakTest { type Err = Error ; fn from_str (line : & str) -> Result < SentenceBreakTest , Error > { let (groups , comment) = parse_break_test (line) ? ; Ok (SentenceBreakTest { sentences : groups , comment }) } }
};
}
