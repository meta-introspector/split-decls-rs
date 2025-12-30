// Generated macro for CACHED_ENV_VARS (static)
macro_rules! Depcrate_compiler_cCACHED_ENV_VARS {
() => {
// Module: crate::compiler::c
// Provides: {"CACHED_ENV_VARS"}
// Dependencies: {}
# [doc = " Environment variables that are factored into the cache key."] static CACHED_ENV_VARS : LazyLock < HashSet < & 'static OsStr > > = LazyLock :: new (| | { ["SCCACHE_C_CUSTOM_CACHE_BUSTER" , "MACOSX_DEPLOYMENT_TARGET" , "IPHONEOS_DEPLOYMENT_TARGET" , "TVOS_DEPLOYMENT_TARGET" , "WATCHOS_DEPLOYMENT_TARGET" , "SDKROOT" , "CCC_OVERRIDE_OPTIONS" ,] . iter () . map (OsStr :: new) . collect () }) ;
};
}
