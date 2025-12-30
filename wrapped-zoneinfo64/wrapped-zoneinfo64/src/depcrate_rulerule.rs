// Generated macro for Rule (struct)
macro_rules! Depcrate_ruleRule {
() => {
// Module: crate::rule
// Provides: {"Rule"}
// Dependencies: {}
# [derive (Debug , Clone , Copy , PartialEq)] pub (crate) struct Rule < 'a > { # [doc = " The year the rule starts applying"] pub (crate) start_year : i32 , # [doc = " The offset of standard time"] pub (crate) standard_offset_seconds : i32 , pub (crate) inner : & 'a TzRule , }
};
}
