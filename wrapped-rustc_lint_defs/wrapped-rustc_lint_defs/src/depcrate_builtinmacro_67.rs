// Generated macro for macro_67 (macro)
macro_rules! Depcrate_builtinmacro_67 {
() => {
// Module: crate::builtin
// Provides: {"macro_67"}
// Dependencies: {}
declare_lint ! { # [doc = " The `elided_lifetimes_in_paths` lint detects the use of hidden"] # [doc = " lifetime parameters."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,compile_fail"] # [doc = " #![deny(elided_lifetimes_in_paths)]"] # [doc = " #![deny(warnings)]"] # [doc = " struct Foo<'a> {"] # [doc = "     x: &'a u32"] # [doc = " }"] # [doc = ""] # [doc = " fn foo(x: &Foo) {"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Elided lifetime parameters can make it difficult to see at a glance"] # [doc = " that borrowing is occurring. This lint ensures that lifetime"] # [doc = " parameters are always explicitly stated, even if it is the `'_`"] # [doc = " [placeholder lifetime]."] # [doc = ""] # [doc = " This lint is \"allow\" by default because it has some known issues, and"] # [doc = " may require a significant transition for old code."] # [doc = ""] # [doc = " [placeholder lifetime]: https://doc.rust-lang.org/reference/lifetime-elision.html#lifetime-elision-in-functions"] pub ELIDED_LIFETIMES_IN_PATHS , Allow , "hidden lifetime parameters in types are deprecated" }
};
}
