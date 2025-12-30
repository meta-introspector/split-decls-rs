// Generated macro for impl_34 (impl)
macro_rules! Depcrate_interest_cacheimpl_34 {
() => {
// Module: crate::interest_cache
// Provides: {"impl_34"}
// Dependencies: {}
impl State { fn new (epoch : usize , config : & InterestCacheConfig) -> Self { State { epoch , min_verbosity : config . min_verbosity , cache : LruCache :: new (config . lru_cache_size) , } } }
};
}
