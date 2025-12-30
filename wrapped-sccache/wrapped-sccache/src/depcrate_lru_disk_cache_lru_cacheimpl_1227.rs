// Generated macro for impl_1227 (impl)
macro_rules! Depcrate_lru_disk_cache_lru_cacheimpl_1227 {
() => {
// Module: crate::lru_disk_cache::lru_cache
// Provides: {"impl_1227"}
// Dependencies: {}
impl < 'a , K , V > Iterator for Iter < 'a , K , V > { type Item = (& 'a K , & 'a V) ; fn next (& mut self) -> Option < (& 'a K , & 'a V) > { self . 0 . next () } fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . size_hint () } }
};
}
