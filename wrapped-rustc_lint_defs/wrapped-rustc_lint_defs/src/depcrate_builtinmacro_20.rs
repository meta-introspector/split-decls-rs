// Generated macro for macro_20 (macro)
macro_rules! Depcrate_builtinmacro_20 {
() => {
// Module: crate::builtin
// Provides: {"macro_20"}
// Dependencies: {}
declare_lint ! { # [doc = " The `conflicting_repr_hints` lint detects [`repr` attributes] with"] # [doc = " conflicting hints."] # [doc = ""] # [doc = " [`repr` attributes]: https://doc.rust-lang.org/reference/type-layout.html#representations"] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,compile_fail"] # [doc = " #[repr(u32, u64)]"] # [doc = " enum Foo {"] # [doc = "     Variant1,"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " The compiler incorrectly accepted these conflicting representations in"] # [doc = " the past. This is a [future-incompatible] lint to transition this to a"] # [doc = " hard error in the future. See [issue #68585] for more details."] # [doc = ""] # [doc = " To correct the issue, remove one of the conflicting hints."] # [doc = ""] # [doc = " [issue #68585]: https://github.com/rust-lang/rust/issues/68585"] # [doc = " [future-incompatible]: ../index.md#future-incompatible-lints"] pub CONFLICTING_REPR_HINTS , Deny , "conflicts between `#[repr(..)]` hints that were previously accepted and used in practice" , @ future_incompatible = FutureIncompatibleInfo { reason : FutureIncompatibilityReason :: FutureReleaseError , reference : "issue #68585 <https://github.com/rust-lang/rust/issues/68585>" , report_in_deps : true , } ; }
};
}
