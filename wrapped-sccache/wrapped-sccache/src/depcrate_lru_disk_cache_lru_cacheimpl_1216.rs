// Generated macro for impl_1216 (impl)
macro_rules! Depcrate_lru_disk_cache_lru_cacheimpl_1216 {
() => {
// Module: crate::lru_disk_cache::lru_cache
// Provides: {"impl_1216"}
// Dependencies: {}
impl < K : Eq + Hash , V , S : BuildHasher , M : CountableMeter < K , V > > Extend < (K , V) > for LruCache < K , V , S , M > { fn extend < I : IntoIterator < Item = (K , V) > > (& mut self , iter : I) { for (k , v) in iter { self . insert (k , v) ; } } }
};
}
