// Generated macro for impl_230 (impl)
macro_rules! Depcrate_grapheme_cluster_breakimpl_230 {
() => {
// Module: crate::grapheme_cluster_break
// Provides: {"impl_230"}
// Dependencies: {}
impl std :: str :: FromStr for GraphemeClusterBreak { type Err = Error ; fn from_str (line : & str) -> Result < GraphemeClusterBreak , Error > { let (codepoints , value) = parse_codepoint_association (line) ? ; Ok (GraphemeClusterBreak { codepoints , value : value . to_string () }) } }
};
}
