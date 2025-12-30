// Generated macro for FloatLiteralTrailingZero (enum)
macro_rules! Depcrate_config_optionsFloatLiteralTrailingZero {
() => {
// Module: crate::config::options
// Provides: {"FloatLiteralTrailingZero"}
// Dependencies: {}
# [doc = " How to treat trailing zeros in floating-point literals."] # [config_type] pub enum FloatLiteralTrailingZero { # [doc = " Leave the literal as-is."] Preserve , # [doc = " Add a trailing zero to the literal."] Always , # [doc = " Add a trailing zero by default. If the literal contains an exponent or a suffix, the zero"] # [doc = " and the preceding period are removed."] IfNoPostfix , # [doc = " Remove the trailing zero. If the literal contains an exponent or a suffix, the preceding"] # [doc = " period is also removed."] Never , }
};
}
