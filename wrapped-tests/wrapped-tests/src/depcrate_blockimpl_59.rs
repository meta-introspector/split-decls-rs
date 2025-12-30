// Generated macro for impl_59 (impl)
macro_rules! Depcrate_blockimpl_59 {
() => {
// Module: crate::block
// Provides: {"impl_59"}
// Dependencies: {}
impl Clone for CloneDropTracker { fn clone (& self) -> Self { COUNT . with_borrow_mut (| count | { count . clone += 1 ; }) ; Self (()) } }
};
}
