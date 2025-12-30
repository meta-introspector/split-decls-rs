// Generated macro for macro_77 (macro)
macro_rules! Depcrate_builtinmacro_77 {
() => {
// Module: crate::builtin
// Provides: {"macro_77"}
// Dependencies: {}
declare_lint ! { # [doc = " The `deprecated_in_future` lint is internal to rustc and should not be"] # [doc = " used by user code."] # [doc = ""] # [doc = " This lint is only enabled in the standard library. It works with the"] # [doc = " use of `#[deprecated]` with a `since` field of a version in the future."] # [doc = " This allows something to be marked as deprecated in a future version,"] # [doc = " and then this lint will ensure that the item is no longer used in the"] # [doc = " standard library. See the [stability documentation] for more details."] # [doc = ""] # [doc = " [stability documentation]: https://rustc-dev-guide.rust-lang.org/stability.html#deprecated"] pub DEPRECATED_IN_FUTURE , Allow , "detects use of items that will be deprecated in a future version" , report_in_external_macro }
};
}
