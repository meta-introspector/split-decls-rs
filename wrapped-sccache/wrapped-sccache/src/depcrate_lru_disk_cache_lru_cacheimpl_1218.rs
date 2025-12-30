// Generated macro for impl_1218 (impl)
macro_rules! Depcrate_lru_disk_cache_lru_cacheimpl_1218 {
() => {
// Module: crate::lru_disk_cache::lru_cache
// Provides: {"impl_1218"}
// Dependencies: {}
impl < K : Eq + Hash , V , S : BuildHasher , M : CountableMeter < K , V > > IntoIterator for LruCache < K , V , S , M > { type Item = (K , V) ; type IntoIter = IntoIter < K , V > ; fn into_iter (self) -> IntoIter < K , V > { IntoIter (self . map . into_iter ()) } }
};
}
