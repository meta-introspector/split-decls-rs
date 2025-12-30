// Generated macro for macro_115 (macro)
macro_rules! Depcrate_builtinmacro_115 {
() => {
// Module: crate::builtin
// Provides: {"macro_115"}
// Dependencies: {}
declare_lint ! { # [doc = " The `break_with_label_and_loop` lint detects labeled `break` expressions with"] # [doc = " an unlabeled loop as their value expression."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " 'label: loop {"] # [doc = "     break 'label loop { break 42; };"] # [doc = " };"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " In Rust, loops can have a label, and `break` expressions can refer to that label to"] # [doc = " break out of specific loops (and not necessarily the innermost one). `break` expressions"] # [doc = " can also carry a value expression, which can be another loop. A labeled `break` with an"] # [doc = " unlabeled loop as its value expression is easy to confuse with an unlabeled break with"] # [doc = " a labeled loop and is thus discouraged (but allowed for compatibility); use parentheses"] # [doc = " around the loop expression to silence this warning. Unlabeled `break` expressions with"] # [doc = " labeled loops yield a hard error, which can also be silenced by wrapping the expression"] # [doc = " in parentheses."] pub BREAK_WITH_LABEL_AND_LOOP , Warn , "`break` expression with label and unlabeled loop as value expression" }
};
}
