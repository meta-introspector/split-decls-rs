// Generated macro for TzRule (struct)
macro_rules! Depcrate_ruleTzRule {
() => {
// Module: crate::rule
// Provides: {"TzRule"}
// Dependencies: {}
# [derive (Debug , PartialEq)] pub (crate) struct TzRule { # [doc = " The amount of seconds to add to standard_offset_seconds"] # [doc = " to get the rule offset"] additional_offset_secs : i32 , # [doc = " The yearly start date of the rule"] start : TzRuleDate , # [doc = " The yearly end date of the rule"] end : TzRuleDate , }
};
}
