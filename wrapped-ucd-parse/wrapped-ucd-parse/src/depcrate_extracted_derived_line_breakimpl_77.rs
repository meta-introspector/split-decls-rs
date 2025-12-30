// Generated macro for impl_77 (impl)
macro_rules! Depcrate_extracted_derived_line_breakimpl_77 {
() => {
// Module: crate::extracted::derived_line_break
// Provides: {"impl_77"}
// Dependencies: {}
impl std :: str :: FromStr for DerivedLineBreak { type Err = Error ; fn from_str (line : & str) -> Result < DerivedLineBreak , Error > { let (codepoints , line_break) = parse_codepoint_association (line) ? ; Ok (DerivedLineBreak { codepoints , line_break : line_break . to_string () }) } }
};
}
