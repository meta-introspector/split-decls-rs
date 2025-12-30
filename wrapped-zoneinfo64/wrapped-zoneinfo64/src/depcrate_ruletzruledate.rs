// Generated macro for TzRuleDate (struct)
macro_rules! Depcrate_ruleTzRuleDate {
() => {
// Module: crate::rule
// Provides: {"TzRuleDate"}
// Dependencies: {}
# [derive (Debug , PartialEq)] struct TzRuleDate { # [doc = " A 1-indexed day number"] day : u8 , # [doc = " A day of the week (0 = Sunday)"] day_of_week : u8 , # [doc = " A 1-indexed month number"] month : u8 , # [doc = " The time in the day (in seconds) that the transition occurs"] transition_time : u32 , # [doc = " How to interpret transition_time"] time_mode : TimeMode , # [doc = " How to interpret day, day_of_week, and month"] mode : RuleMode , }
};
}
