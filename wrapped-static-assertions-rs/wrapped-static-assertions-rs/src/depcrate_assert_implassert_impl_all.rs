// Generated macro for assert_impl_all (macro)
macro_rules! Depcrate_assert_implassert_impl_all {
() => {
// Module: crate::assert_impl
// Provides: {"assert_impl_all"}
// Dependencies: {}
# [doc = " Asserts that the type implements _all_ of the given traits."] # [doc = ""] # [doc = " See [`assert_impl_not_all!`] for achieving the opposite effect."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " This can be used to ensure types implement auto traits such as [`Send`] and"] # [doc = " [`Sync`], as well as traits with [blanket `impl`s][blanket]."] # [doc = ""] # [doc = " ```"] # [doc = " # #[macro_use] extern crate static_assertions; fn main() {}"] # [doc = " assert_impl_all!(u32: Copy, Send);"] # [doc = " assert_impl_all!(&str: Into<String>);"] # [doc = " ```"] # [doc = ""] # [doc = " The following example fails to compile because raw pointers do not implement"] # [doc = " [`Send`] since they cannot be moved between threads safely:"] # [doc = ""] # [doc = " ```compile_fail"] # [doc = " # #[macro_use] extern crate static_assertions; fn main() {}"] # [doc = " assert_impl_all!(*const u8: Send);"] # [doc = " ```"] # [doc = ""] # [doc = " [`assert_impl_not_all!`]: macro.assert_not_impl_all.html"] # [doc = " [`Send`]: https://doc.rust-lang.org/std/marker/trait.Send.html"] # [doc = " [`Sync`]: https://doc.rust-lang.org/std/marker/trait.Sync.html"] # [doc = " [blanket]: https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods"] # [macro_export (local_inner_macros)] macro_rules ! assert_impl_all { ($ ty : ty : $ ($ traits : path) ,+ $ (,) ?) => { assert_impl ! ($ ty : $ (($ traits)) &+) ; } ; }
};
}
