// Generated macro for impl_150 (impl)
macro_rules! Depcrate_itemimpl_150 {
() => {
// Module: crate::item
// Provides: {"impl_150"}
// Dependencies: {}
impl Item { # [doc = " Sets `self` to the given item if `self` is none and"] # [doc = " returns a mutable reference to `self`."] pub fn or_insert (& mut self , item : Self) -> & mut Self { if self . is_none () { * self = item ; } self } }
};
}
