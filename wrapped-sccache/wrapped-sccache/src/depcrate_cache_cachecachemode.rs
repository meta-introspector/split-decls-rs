// Generated macro for CacheMode (enum)
macro_rules! Depcrate_cache_cacheCacheMode {
() => {
// Module: crate::cache::cache
// Provides: {"CacheMode"}
// Dependencies: {}
# [doc = " CacheMode is used to represent which mode we are using."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub enum CacheMode { # [doc = " Only read cache from storage."] ReadOnly , # [doc = " Full support of cache storage: read and write."] ReadWrite , }
};
}
