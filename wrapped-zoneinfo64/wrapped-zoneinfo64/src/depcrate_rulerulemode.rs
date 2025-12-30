// Generated macro for RuleMode (enum)
macro_rules! Depcrate_ruleRuleMode {
() => {
// Module: crate::rule
// Provides: {"RuleMode"}
// Dependencies: {}
# [derive (Debug , PartialEq , Copy , Clone)] # [allow (non_camel_case_types , clippy :: upper_case_acronyms)] # [doc = " How to interpret `{day}` `{day_of_week}` and `{month}`"] enum RuleMode { # [doc = " The {day}th {day_of_week} in {month}"] # [doc = ""] # [doc = " Current zoneinfo64 does not use this, instead"] # [doc = " choosing to represent this as DOW_GEQ_DOM with day = 1/8/15/22"] DOW_IN_MONTH , # [doc = " {month} {day}"] # [doc = ""] # [doc = " Current zoneinfo64 does not use this"] DOM , # [doc = " The first {day_of_week} on or after {month} {day}"] DOW_GEQ_DOM , # [doc = " The first {day_of_week} on or before {month} {day}"] # [doc = ""] # [doc = " Typically, this represents rules like \"Last Sunday in March\" (Europe/London)"] DOW_LEQ_DOM , }
};
}
