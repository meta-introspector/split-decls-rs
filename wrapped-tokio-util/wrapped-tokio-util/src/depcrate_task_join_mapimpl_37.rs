// Generated macro for impl_37 (impl)
macro_rules! Depcrate_task_join_mapimpl_37 {
() => {
// Module: crate::task::join_map
// Provides: {"impl_37"}
// Dependencies: {}
impl < 'a , K , V > Iterator for JoinMapKeys < 'a , K , V > { type Item = & 'a K ; fn next (& mut self) -> Option < & 'a K > { self . iter . next () . map (| (key , _) | key) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
