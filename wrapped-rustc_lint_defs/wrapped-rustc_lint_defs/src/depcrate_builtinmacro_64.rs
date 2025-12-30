// Generated macro for macro_64 (macro)
macro_rules! Depcrate_builtinmacro_64 {
() => {
// Module: crate::builtin
// Provides: {"macro_64"}
// Dependencies: {}
declare_lint ! { # [doc = " The `unused_lifetimes` lint detects lifetime parameters that are never"] # [doc = " used."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,compile_fail"] # [doc = " #[deny(unused_lifetimes)]"] # [doc = ""] # [doc = " pub fn foo<'a>() {}"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Unused lifetime parameters may signal a mistake or unfinished code."] # [doc = " Consider removing the parameter."] pub UNUSED_LIFETIMES , Allow , "detects lifetime parameters that are never used" }
};
}
