// Generated macro for impl_111 (impl)
macro_rules! Depcrate_indeximpl_111 {
() => {
// Module: crate::index
// Provides: {"impl_111"}
// Dependencies: {}
impl < 's > ops :: IndexMut < & 's str > for Table { fn index_mut (& mut self , key : & 's str) -> & mut Item { self . entry (key) . or_insert (Item :: None) } }
};
}
