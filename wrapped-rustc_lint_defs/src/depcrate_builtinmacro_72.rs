// Generated macro for macro_72 (macro)
macro_rules! Depcrate_builtinmacro_72 {
() => {
// Module: crate::builtin
// Provides: {"macro_72"}
// Dependencies: {}
declare_lint ! { # [doc = " The `unused_labels` lint detects [labels] that are never used."] # [doc = ""] # [doc = " [labels]: https://doc.rust-lang.org/reference/expressions/loop-expr.html#loop-labels"] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " 'unused_label: loop {}"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Unused labels may signal a mistake or unfinished code. To silence the"] # [doc = " warning for the individual label, prefix it with an underscore such as"] # [doc = " `'_my_label:`."] pub UNUSED_LABELS , Warn , "detects labels that are never used" }
};
}
