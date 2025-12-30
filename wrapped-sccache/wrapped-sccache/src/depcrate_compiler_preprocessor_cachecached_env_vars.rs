// Generated macro for CACHED_ENV_VARS (static)
macro_rules! Depcrate_compiler_preprocessor_cacheCACHED_ENV_VARS {
() => {
// Module: crate::compiler::preprocessor_cache
// Provides: {"CACHED_ENV_VARS"}
// Dependencies: {}
# [doc = " Environment variables that are factored into the preprocessor cache entry cached key."] static CACHED_ENV_VARS : LazyLock < HashSet < & 'static OsStr > > = LazyLock :: new (| | { ["SCCACHE_C_CUSTOM_CACHE_BUSTER" , "CPATH" , "C_INCLUDE_PATH" , "CPLUS_INCLUDE_PATH" , "OBJC_INCLUDE_PATH" , "OBJCPLUS_INCLUDE_PATH" ,] . iter () . map (OsStr :: new) . collect () }) ;
};
}
