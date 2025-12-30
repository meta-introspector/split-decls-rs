// Generated macro for impl_17 (impl)
macro_rules! Depcrate_cachedimpl_17 {
() => {
// Module: crate::cached
// Provides: {"impl_17"}
// Dependencies: {}
impl < T : Send > Iterator for CachedIntoIter < T > { type Item = T ; # [inline] fn next (& mut self) -> Option < T > { self . inner . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } }
};
}
