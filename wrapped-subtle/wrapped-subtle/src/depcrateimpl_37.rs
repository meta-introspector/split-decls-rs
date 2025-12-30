// Generated macro for impl_37 (impl)
macro_rules! Depcrateimpl_37 {
() => {
// Module: crate
// Provides: {"impl_37"}
// Dependencies: {}
# [doc = " `Ordering` is `#[repr(i8)]` where:"] # [doc = ""] # [doc = " - `Less` => -1"] # [doc = " - `Equal` => 0"] # [doc = " - `Greater` => 1"] # [doc = ""] # [doc = " Given this, it's possible to operate on orderings as if they're integers,"] # [doc = " which allows leveraging conditional masking for predication."] impl ConditionallySelectable for cmp :: Ordering { fn conditional_select (a : & Self , b : & Self , choice : Choice) -> Self { let a = * a as i8 ; let b = * b as i8 ; let ret = i8 :: conditional_select (& a , & b , choice) ; unsafe { * ((& ret as * const _) as * const cmp :: Ordering) } } }
};
}
