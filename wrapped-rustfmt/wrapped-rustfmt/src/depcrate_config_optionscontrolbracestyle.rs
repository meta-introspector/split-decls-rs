// Generated macro for ControlBraceStyle (enum)
macro_rules! Depcrate_config_optionsControlBraceStyle {
() => {
// Module: crate::config::options
// Provides: {"ControlBraceStyle"}
// Dependencies: {}
# [config_type] # [doc = " Where to put the opening brace of conditional expressions (`if`, `match`, etc.)."] pub enum ControlBraceStyle { # [doc = " K&R style, Rust community default"] AlwaysSameLine , # [doc = " Stroustrup style"] ClosingNextLine , # [doc = " Allman style"] AlwaysNextLine , }
};
}
