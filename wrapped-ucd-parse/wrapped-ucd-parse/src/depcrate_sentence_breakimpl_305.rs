// Generated macro for impl_305 (impl)
macro_rules! Depcrate_sentence_breakimpl_305 {
() => {
// Module: crate::sentence_break
// Provides: {"impl_305"}
// Dependencies: {}
impl std :: str :: FromStr for SentenceBreak { type Err = Error ; fn from_str (line : & str) -> Result < SentenceBreak , Error > { let (codepoints , value) = parse_codepoint_association (line) ? ; Ok (SentenceBreak { codepoints , value : value . to_string () }) } }
};
}
