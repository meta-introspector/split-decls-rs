// Generated macro for TextRange (struct)
macro_rules! Depcrate_rangeTextRange {
() => {
// Module: crate::range
// Provides: {"TextRange"}
// Dependencies: {}
# [doc = " A range in text, represented as a pair of [`TextSize`][struct@TextSize]."] # [doc = ""] # [doc = " It is a logic error for `start` to be greater than `end`."] # [derive (Default , Copy , Clone , Eq , PartialEq , Hash)] pub struct TextRange { start : TextSize , end : TextSize , }
};
}
