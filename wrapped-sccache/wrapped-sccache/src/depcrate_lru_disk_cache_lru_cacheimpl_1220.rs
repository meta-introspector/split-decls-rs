// Generated macro for impl_1220 (impl)
macro_rules! Depcrate_lru_disk_cache_lru_cacheimpl_1220 {
() => {
// Module: crate::lru_disk_cache::lru_cache
// Provides: {"impl_1220"}
// Dependencies: {}
impl < 'a , K : Eq + Hash , V , S : BuildHasher , M : CountableMeter < K , V > > IntoIterator for & 'a mut LruCache < K , V , S , M > { type Item = (& 'a K , & 'a mut V) ; type IntoIter = IterMut < 'a , K , V > ; fn into_iter (self) -> IterMut < 'a , K , V > { self . internal_iter_mut () } }
};
}
