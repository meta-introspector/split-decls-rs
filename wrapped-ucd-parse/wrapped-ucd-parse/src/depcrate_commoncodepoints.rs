// Generated macro for Codepoints (enum)
macro_rules! Depcrate_commonCodepoints {
() => {
// Module: crate::common
// Provides: {"Codepoints"}
// Dependencies: {}
# [doc = " A representation of either a single codepoint or a range of codepoints."] # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq , PartialOrd , Ord)] pub enum Codepoints { # [doc = " A single codepoint."] Single (Codepoint) , # [doc = " A range of codepoints."] Range (CodepointRange) , }
};
}
