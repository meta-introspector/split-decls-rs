// Generated macro for assert_impl_any (macro)
macro_rules! Depcrate_assert_implassert_impl_any {
() => {
// Module: crate::assert_impl
// Provides: {"assert_impl_any"}
// Dependencies: {}
# [doc = " Asserts that the type implements _any_ of the given traits."] # [doc = ""] # [doc = " See [`assert_impl_not_any!`] for achieving the opposite effect."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " `u8` cannot be converted from `u16`, but it can be converted into `u16`:"] # [doc = ""] # [doc = " ```"] # [doc = " # #[macro_use] extern crate static_assertions; fn main() {}"] # [doc = " assert_impl_any!(u8: From<u16>, Into<u16>);"] # [doc = " ```"] # [doc = ""] # [doc = " The unit type cannot be converted from `u8` or `u16`, but it does implement"] # [doc = " [`Send`]:"] # [doc = ""] # [doc = " ```"] # [doc = " # #[macro_use] extern crate static_assertions; fn main() {}"] # [doc = " assert_impl_any!((): From<u8>, From<u16>, Send);"] # [doc = " ```"] # [doc = ""] # [doc = " The following example fails to compile because raw pointers do not implement"] # [doc = " [`Send`] or [`Sync`] since they cannot be moved or shared between threads"] # [doc = " safely:"] # [doc = ""] # [doc = " ```compile_fail"] # [doc = " # #[macro_use] extern crate static_assertions; fn main() {}"] # [doc = " assert_impl_any!(*const u8: Send, Sync);"] # [doc = " ```"] # [doc = ""] # [doc = " [`assert_impl_not_any!`]: macro.assert_not_impl_any.html"] # [doc = " [`Send`]: https://doc.rust-lang.org/std/marker/trait.Send.html"] # [doc = " [`Sync`]: https://doc.rust-lang.org/std/marker/trait.Sync.html"] # [macro_export (local_inner_macros)] macro_rules ! assert_impl_any { ($ ty : ty : $ ($ traits : path) ,+ $ (,) ?) => { assert_impl ! ($ ty : $ (($ traits)) |+) ; } ; }
};
}
