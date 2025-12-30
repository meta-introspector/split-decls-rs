// Generated macro for impl_114 (impl)
macro_rules! Depcrate_indeximpl_114 {
() => {
// Module: crate::index
// Provides: {"impl_114"}
// Dependencies: {}
impl < 's > ops :: Index < & 's str > for DocumentMut { type Output = Item ; fn index (& self , key : & 's str) -> & Item { self . root . index (key) } }
};
}
