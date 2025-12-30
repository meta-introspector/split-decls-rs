// Generated macro for SingleCache (struct)
macro_rules! Depcrate_query_cachesSingleCache {
() => {
// Module: crate::query::caches
// Provides: {"SingleCache"}
// Dependencies: {}
# [doc = " In-memory cache for queries whose key type only has one value (e.g. `()`)."] # [doc = " The cache therefore only needs to store one query return value."] pub struct SingleCache < V > { cache : OnceLock < (V , DepNodeIndex) > , }
};
}
