// Generated macro for macro_62 (macro)
macro_rules! Depcrate_builtinmacro_62 {
() => {
// Module: crate::builtin
// Provides: {"macro_62"}
// Dependencies: {}
declare_lint ! { # [doc = " The `unconditional_recursion` lint detects functions that cannot"] # [doc = " return without calling themselves."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " fn foo() {"] # [doc = "     foo();"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " It is usually a mistake to have a recursive call that does not have"] # [doc = " some condition to cause it to terminate. If you really intend to have"] # [doc = " an infinite loop, using a `loop` expression is recommended."] pub UNCONDITIONAL_RECURSION , Warn , "functions that cannot return without calling themselves" }
};
}
