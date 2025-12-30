// Generated macro for LocatingSlice (struct)
macro_rules! Depcrate_stream_locatingLocatingSlice {
() => {
// Module: crate::stream::locating
// Provides: {"LocatingSlice"}
// Dependencies: {}
# [doc = " Allow collecting the span of a parsed token within a slice"] # [doc = ""] # [doc = " Converting byte offsets to line or column numbers is left up to the user, as computing column"] # [doc = " numbers requires domain knowledge (are columns byte-based, codepoint-based, or grapheme-based?)"] # [doc = " and O(n) iteration over the input to determine codepoint and line boundaries."] # [doc = ""] # [doc = " [The `line-span` crate](https://docs.rs/line-span/latest/line_span/) can help with converting"] # [doc = " byte offsets to line numbers."] # [doc = ""] # [doc = " See [`Parser::span`][crate::Parser::span] and [`Parser::with_span`][crate::Parser::with_span] for more details"] # [derive (Copy , Clone , Default , PartialEq , Eq , PartialOrd , Ord)] # [doc (alias = "LocatingSliceSpan")] # [doc (alias = "Located")] pub struct LocatingSlice < I > { initial : I , input : I , }
};
}
