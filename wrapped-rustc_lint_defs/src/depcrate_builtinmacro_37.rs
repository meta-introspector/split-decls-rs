// Generated macro for macro_37 (macro)
macro_rules! Depcrate_builtinmacro_37 {
() => {
// Module: crate::builtin
// Provides: {"macro_37"}
// Dependencies: {}
declare_lint ! { # [doc = " The `unreachable_code` lint detects unreachable code paths."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " panic!(\"we never go past here!\");"] # [doc = ""] # [doc = " let x = 5;"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Unreachable code may signal a mistake or unfinished code. If the code"] # [doc = " is no longer in use, consider removing it."] pub UNREACHABLE_CODE , Warn , "detects unreachable code paths" , report_in_external_macro }
};
}
