// Generated macro for impl_105 (impl)
macro_rules! Depcrate_cycleimpl_105 {
() => {
// Module: crate::cycle
// Provides: {"impl_105"}
// Dependencies: {}
impl Iterator for CycleHeadIdsIterator < '_ > { type Item = crate :: Id ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () . map (| head | head . database_key_index . key_index ()) } }
};
}
