// Generated macro for Cache (enum)
macro_rules! Depcrate_cache_cacheCache {
() => {
// Module: crate::cache::cache
// Provides: {"Cache"}
// Dependencies: {}
# [doc = " Result of a cache lookup."] pub enum Cache { # [doc = " Result was found in cache."] Hit (CacheRead) , # [doc = " Result was not found in cache."] Miss , # [doc = " Do not cache the results of the compilation."] None , # [doc = " Cache entry should be ignored, force compilation."] Recache , }
};
}
