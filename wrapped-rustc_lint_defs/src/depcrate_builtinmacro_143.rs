// Generated macro for macro_143 (macro)
macro_rules! Depcrate_builtinmacro_143 {
() => {
// Module: crate::builtin
// Provides: {"macro_143"}
// Dependencies: {}
declare_lint ! { # [doc = " The `unsafe_attr_outside_unsafe` lint detects a missing unsafe keyword"] # [doc = " on attributes considered unsafe."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,edition2021"] # [doc = " #![warn(unsafe_attr_outside_unsafe)]"] # [doc = ""] # [doc = " #[no_mangle]"] # [doc = " extern \"C\" fn foo() {}"] # [doc = ""] # [doc = " fn main() {}"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Some attributes (e.g. `no_mangle`, `export_name`, `link_section` -- see"] # [doc = " [issue #82499] for a more complete list) are considered \"unsafe\" attributes."] # [doc = " An unsafe attribute must only be used inside unsafe(...)."] # [doc = ""] # [doc = " This lint can automatically wrap the attributes in `unsafe(...)` , but this"] # [doc = " obviously cannot verify that the preconditions of the `unsafe`"] # [doc = " attributes are fulfilled, so that is still up to the user."] # [doc = ""] # [doc = " The lint is currently \"allow\" by default, but that might change in the"] # [doc = " future."] # [doc = ""] # [doc = " [editions]: https://doc.rust-lang.org/edition-guide/"] # [doc = " [issue #82499]: https://github.com/rust-lang/rust/issues/82499"] pub UNSAFE_ATTR_OUTSIDE_UNSAFE , Allow , "detects unsafe attributes outside of unsafe" , @ future_incompatible = FutureIncompatibleInfo { reason : FutureIncompatibilityReason :: EditionError (Edition :: Edition2024) , reference : "<https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-attributes.html>" , } ; }
};
}
