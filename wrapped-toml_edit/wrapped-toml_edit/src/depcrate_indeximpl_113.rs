// Generated macro for impl_113 (impl)
macro_rules! Depcrate_indeximpl_113 {
() => {
// Module: crate::index
// Provides: {"impl_113"}
// Dependencies: {}
impl < 's > ops :: IndexMut < & 's str > for InlineTable { fn index_mut (& mut self , key : & 's str) -> & mut Value { self . get_mut (key) . expect ("index not found") } }
};
}
