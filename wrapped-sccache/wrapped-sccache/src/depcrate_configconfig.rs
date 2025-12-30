// Generated macro for Config (struct)
macro_rules! Depcrate_configConfig {
() => {
// Module: crate::config
// Provides: {"Config"}
// Dependencies: {}
# [derive (Debug , Default , PartialEq , Eq)] pub struct Config { pub cache : Option < CacheType > , pub fallback_cache : DiskCacheConfig , pub dist : DistConfig , pub server_startup_timeout : Option < std :: time :: Duration > , }
};
}
