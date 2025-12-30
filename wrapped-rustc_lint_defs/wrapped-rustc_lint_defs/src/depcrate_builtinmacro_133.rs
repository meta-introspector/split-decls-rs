// Generated macro for macro_133 (macro)
macro_rules! Depcrate_builtinmacro_133 {
() => {
// Module: crate::builtin
// Provides: {"macro_133"}
// Dependencies: {}
declare_lint ! { # [doc = " The `unknown_diagnostic_attributes` lint detects unknown diagnostic attributes."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " #[diagnostic::does_not_exist]"] # [doc = " struct Thing;"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " It is usually a mistake to specify a diagnostic attribute that does not exist. Check the"] # [doc = " spelling, and check the diagnostic attribute listing for the correct name. Also consider if"] # [doc = " you are using an old version of the compiler and the attribute is only available in a newer"] # [doc = " version. See the [reference] for the list of diagnostic attributes."] # [doc = ""] # [doc = " [reference]: https://doc.rust-lang.org/nightly/reference/attributes/diagnostics.html#the-diagnostic-tool-attribute-namespace"] pub UNKNOWN_DIAGNOSTIC_ATTRIBUTES , Warn , "detects unknown diagnostic attributes" , }
};
}
