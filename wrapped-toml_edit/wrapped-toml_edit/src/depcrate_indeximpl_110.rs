// Generated macro for impl_110 (impl)
macro_rules! Depcrate_indeximpl_110 {
() => {
// Module: crate::index
// Provides: {"impl_110"}
// Dependencies: {}
impl < 's > ops :: Index < & 's str > for Table { type Output = Item ; fn index (& self , key : & 's str) -> & Item { self . get (key) . expect ("index not found") } }
};
}
