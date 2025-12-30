// Generated macro for impl_1231 (impl)
macro_rules! Depcrate_lru_disk_cache_lru_cacheimpl_1231 {
() => {
// Module: crate::lru_disk_cache::lru_cache
// Provides: {"impl_1231"}
// Dependencies: {}
impl < 'a , K , V > Iterator for IterMut < 'a , K , V > { type Item = (& 'a K , & 'a mut V) ; fn next (& mut self) -> Option < (& 'a K , & 'a mut V) > { self . 0 . next () } fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . size_hint () } }
};
}
