// Generated macro for impl_233 (impl)
macro_rules! Depcrate_grapheme_cluster_breakimpl_233 {
() => {
// Module: crate::grapheme_cluster_break
// Provides: {"impl_233"}
// Dependencies: {}
impl std :: str :: FromStr for GraphemeClusterBreakTest { type Err = Error ; fn from_str (line : & str) -> Result < GraphemeClusterBreakTest , Error > { let (groups , comment) = parse_break_test (line) ? ; Ok (GraphemeClusterBreakTest { grapheme_clusters : groups , comment }) } }
};
}
