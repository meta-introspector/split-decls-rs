// Generated macro for does_impl (macro)
macro_rules! Depcrate_does_impldoes_impl {
() => {
// Module: crate::does_impl
// Provides: {"does_impl"}
// Dependencies: {}
# [doc = " Returns `true` if the type does implement a logical trait expression."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " One can mimic `assert_impl!` using this macro:"] # [doc = ""] # [doc = " ```"] # [doc = " # #[macro_use] extern crate static_assertions; fn main() {}"] # [doc = " const CONDITION: bool = does_impl!(u32: From<u8>);"] # [doc = ""] # [doc = " const_assert!(CONDITION);"] # [doc = " ```"] # [macro_export (local_inner_macros)] macro_rules ! does_impl { ($ ty : ty : $ ($ trait_expr : tt) +) => { _does_impl ! ($ ty : $ ($ trait_expr) +) . value () } ; }
};
}
