// Generated macro for Solution (struct)
macro_rules! DepcrateSolution {
() => {
// Module: crate
// Provides: {"Solution"}
// Dependencies: {}
# [doc = " Solution to a diagnostic item."] # [derive (Debug , Clone , Hash , PartialEq , Eq)] pub struct Solution { # [doc = " The error message of the diagnostic item."] pub message : String , # [doc = " Possible solutions to fix the error."] pub replacements : Vec < Replacement > , }
};
}
