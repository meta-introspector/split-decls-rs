// Generated macro for impl_14 (impl)
macro_rules! Depcrate_cachedimpl_14 {
() => {
// Module: crate::cached
// Provides: {"impl_14"}
// Dependencies: {}
impl < 'a , T : Send + 'a > Iterator for CachedIterMut < 'a , T > { type Item = & 'a mut T ; # [inline] fn next (& mut self) -> Option < & 'a mut T > { self . inner . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } }
};
}
