// Generated macro for Transition (struct)
macro_rules! DepcrateTransition {
() => {
// Module: crate
// Provides: {"Transition"}
// Dependencies: {}
# [doc = " A transition"] # [derive (Debug , Clone , Copy , PartialEq)] pub struct Transition { # [doc = " When the transition starts"] pub since : i64 , # [doc = " The offset from UTC after this transition"] pub offset : UtcOffset , # [doc = " Whether or not the rule (i.e. \"non standard\" time) applies"] pub rule_applies : bool , }
};
}
