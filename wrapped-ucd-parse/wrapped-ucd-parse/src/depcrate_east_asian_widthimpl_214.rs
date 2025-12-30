// Generated macro for impl_214 (impl)
macro_rules! Depcrate_east_asian_widthimpl_214 {
() => {
// Module: crate::east_asian_width
// Provides: {"impl_214"}
// Dependencies: {}
impl std :: str :: FromStr for EastAsianWidth { type Err = Error ; fn from_str (line : & str) -> Result < EastAsianWidth , Error > { let (codepoints , width) = parse_codepoint_association (line) ? ; Ok (EastAsianWidth { codepoints , width : width . to_string () }) } }
};
}
