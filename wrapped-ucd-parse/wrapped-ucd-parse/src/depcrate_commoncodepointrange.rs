// Generated macro for CodepointRange (struct)
macro_rules! Depcrate_commonCodepointRange {
() => {
// Module: crate::common
// Provides: {"CodepointRange"}
// Dependencies: {}
# [doc = " A range of Unicode codepoints. The range is inclusive; both ends of the"] # [doc = " range are guaranteed to be valid codepoints."] # [derive (Clone , Copy , Debug , Default , Eq , Hash , PartialEq , PartialOrd , Ord ,)] pub struct CodepointRange { # [doc = " The start of the codepoint range."] pub start : Codepoint , # [doc = " The end of the codepoint range."] pub end : Codepoint , }
};
}
