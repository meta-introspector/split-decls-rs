// Generated macro for macro_131 (macro)
macro_rules! Depcrate_builtinmacro_131 {
() => {
// Module: crate::builtin
// Provides: {"macro_131"}
// Dependencies: {}
declare_lint ! { # [doc = " The `malformed_diagnostic_attributes` lint detects malformed diagnostic attributes."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " #[diagnostic::do_not_recommend(message = \"message\")]"] # [doc = " trait Trait {}"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " It is usually a mistake to use options or syntax that is not supported. Check the spelling,"] # [doc = " and check the diagnostic attribute listing for the correct name and syntax. Also consider if"] # [doc = " you are using an old version of the compiler; perhaps the option or syntax is only available"] # [doc = " in a newer version. See the [reference] for a list of diagnostic attributes and the syntax"] # [doc = " of each."] # [doc = ""] # [doc = " [reference]: https://doc.rust-lang.org/nightly/reference/attributes/diagnostics.html#the-diagnostic-tool-attribute-namespace"] pub MALFORMED_DIAGNOSTIC_ATTRIBUTES , Warn , "detects malformed diagnostic attributes" , }
};
}
