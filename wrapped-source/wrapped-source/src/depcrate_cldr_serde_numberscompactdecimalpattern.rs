// Generated macro for CompactDecimalPattern (struct)
macro_rules! Depcrate_cldr_serde_numbersCompactDecimalPattern {
() => {
// Module: crate::cldr_serde::numbers
// Provides: {"CompactDecimalPattern"}
// Dependencies: {}
# [derive (PartialEq , Debug , Default)] pub (crate) struct CompactDecimalPattern { # [doc = " The magnitude part of the pattern key."] # [doc = ""] # [doc = " Examples:"] # [doc = " - \"1000000-count-zero\" --> \"1000000\""] pub (crate) magnitude : String , # [doc = " The count part of the pattern key."] # [doc = ""] # [doc = " Examples:"] # [doc = " - \"1000000-count-zero\" --> \"zero\""] pub (crate) count : String , # [doc = " The pattern value."] # [doc = ""] # [doc = " Examples:"] # [doc = " - \"1000-count-one\": \"¤0K\" --> \"¤0K\""] pub (crate) pattern : String , }
};
}
