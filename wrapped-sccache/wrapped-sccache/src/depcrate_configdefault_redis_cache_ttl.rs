// Generated macro for DEFAULT_REDIS_CACHE_TTL (const)
macro_rules! Depcrate_configDEFAULT_REDIS_CACHE_TTL {
() => {
// Module: crate::config
// Provides: {"DEFAULT_REDIS_CACHE_TTL"}
// Dependencies: {}
# [doc = " redis has no default TTL - all caches live forever"] # [doc = ""] # [doc = " We keep the TTL as 0 here as redis does"] # [doc = ""] # [doc = " Please change this value freely if we have a better choice."] const DEFAULT_REDIS_CACHE_TTL : u64 = 0 ;
};
}
