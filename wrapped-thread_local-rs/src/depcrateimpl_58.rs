// Generated macro for impl_58 (impl)
macro_rules! Depcrateimpl_58 {
() => {
// Module: crate
// Provides: {"impl_58"}
// Dependencies: {}
impl < 'a , T : Send + Sync > Iterator for Iter < 'a , T > { type Item = & 'a T ; fn next (& mut self) -> Option < Self :: Item > { self . raw . next (self . thread_local) } fn size_hint (& self) -> (usize , Option < usize >) { self . raw . size_hint (self . thread_local) } }
};
}
