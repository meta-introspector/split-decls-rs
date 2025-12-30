// Generated macro for impl_1219 (impl)
macro_rules! Depcrate_lru_disk_cache_lru_cacheimpl_1219 {
() => {
// Module: crate::lru_disk_cache::lru_cache
// Provides: {"impl_1219"}
// Dependencies: {}
impl < 'a , K : Eq + Hash , V , S : BuildHasher , M : CountableMeter < K , V > > IntoIterator for & 'a LruCache < K , V , S , M > { type Item = (& 'a K , & 'a V) ; type IntoIter = Iter < 'a , K , V > ; fn into_iter (self) -> Iter < 'a , K , V > { self . iter () } }
};
}
