// Generated macro for macro_80 (macro)
macro_rules! Depcrate_builtinmacro_80 {
() => {
// Module: crate::builtin
// Provides: {"macro_80"}
// Dependencies: {}
declare_lint ! { # [doc = " The `inline_no_sanitize` lint detects incompatible use of"] # [doc = " [`#[inline(always)]`][inline] and [`#[sanitize(xyz = \"off\")]`][sanitize]."] # [doc = ""] # [doc = " [inline]: https://doc.rust-lang.org/reference/attributes/codegen.html#the-inline-attribute"] # [doc = " [sanitize]: https://doc.rust-lang.org/nightly/unstable-book/language-features/no-sanitize.html"] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " #![cfg_attr(not(bootstrap), feature(sanitize))]"] # [doc = ""] # [doc = " #[inline(always)]"] # [doc = " #[cfg_attr(not(bootstrap), sanitize(address = \"off\"))]"] # [doc = " fn x() {}"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     x()"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " The use of the [`#[inline(always)]`][inline] attribute prevents the"] # [doc = " the [`#[sanitize(xyz = \"off\")]`][sanitize] attribute from working."] # [doc = " Consider temporarily removing `inline` attribute."] pub INLINE_NO_SANITIZE , Warn , r#"detects incompatible use of `#[inline(always)]` and `#[sanitize(... = "off")]`"# , }
};
}
