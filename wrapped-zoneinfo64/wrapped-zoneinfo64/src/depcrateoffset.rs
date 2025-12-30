// Generated macro for Offset (struct)
macro_rules! DepcrateOffset {
() => {
// Module: crate
// Provides: {"Offset"}
// Dependencies: {}
# [doc = " A resolved offset for a given point in time"] # [derive (Debug , Clone , Copy , PartialEq , Default)] pub struct Offset { # [doc = " The offset from UTC of this time zone"] pub offset : UtcOffset , # [doc = " Whether or not the Rule (i.e. \"non standard\" time) applies"] pub rule_applies : bool , }
};
}
