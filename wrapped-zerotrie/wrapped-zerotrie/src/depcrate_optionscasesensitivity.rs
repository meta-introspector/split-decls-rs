// Generated macro for CaseSensitivity (enum)
macro_rules! Depcrate_optionsCaseSensitivity {
() => {
// Module: crate::options
// Provides: {"CaseSensitivity"}
// Dependencies: {}
# [doc = " How to handle strings with mixed ASCII case at a node, such as \"abc\" and \"Abc\""] # [derive (Copy , Clone)] pub (crate) enum CaseSensitivity { # [doc = " Allow all strings and sort them by byte value."] Sensitive , # [doc = " Reject strings with different case and sort them as if `to_ascii_lowercase` is called."] IgnoreCase , }
};
}
