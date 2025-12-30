// Generated macro for impl_1217 (impl)
macro_rules! Depcrate_lru_disk_cache_lru_cacheimpl_1217 {
() => {
// Module: crate::lru_disk_cache::lru_cache
// Provides: {"impl_1217"}
// Dependencies: {}
impl < K : fmt :: Debug + Eq + Hash , V : fmt :: Debug , S : BuildHasher , M : CountableMeter < K , V > > fmt :: Debug for LruCache < K , V , S , M > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_map () . entries (self . iter () . rev ()) . finish () } }
};
}
