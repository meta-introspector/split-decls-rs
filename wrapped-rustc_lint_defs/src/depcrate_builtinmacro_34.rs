// Generated macro for macro_34 (macro)
macro_rules! Depcrate_builtinmacro_34 {
() => {
// Module: crate::builtin
// Provides: {"macro_34"}
// Dependencies: {}
declare_lint ! { # [doc = " The `unused_assignments` lint detects assignments that will never be read."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " let mut x = 5;"] # [doc = " x = 6;"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Unused assignments may signal a mistake or unfinished code. If the"] # [doc = " variable is never used after being assigned, then the assignment can"] # [doc = " be removed. Variables with an underscore prefix such as `_x` will not"] # [doc = " trigger this lint."] pub UNUSED_ASSIGNMENTS , Warn , "detect assignments that will never be read" }
};
}
