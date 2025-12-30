// Generated macro for impl_60 (impl)
macro_rules! Depcrate_blockimpl_60 {
() => {
// Module: crate::block
// Provides: {"impl_60"}
// Dependencies: {}
impl Drop for CloneDropTracker { fn drop (& mut self) { COUNT . with_borrow_mut (| count | { count . drop += 1 ; }) ; } }
};
}
