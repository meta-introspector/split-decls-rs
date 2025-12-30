// Generated macro for impl_990 (impl)
macro_rules! Depcrate_configimpl_990 {
() => {
// Module: crate::config
// Provides: {"impl_990"}
// Dependencies: {}
impl Default for DiskCacheConfig { fn default () -> Self { DiskCacheConfig { dir : default_disk_cache_dir () , size : default_disk_cache_size () , preprocessor_cache_mode : PreprocessorCacheModeConfig :: activated () , rw_mode : CacheModeConfig :: ReadWrite , } } }
};
}
