// Generated macro for DiskCacheConfig (struct)
macro_rules! Depcrate_configDiskCacheConfig {
() => {
// Module: crate::config
// Provides: {"DiskCacheConfig"}
// Dependencies: {}
# [derive (Debug , PartialEq , Eq , Serialize , Deserialize)] # [serde (deny_unknown_fields)] # [serde (default)] pub struct DiskCacheConfig { pub dir : PathBuf , # [serde (deserialize_with = "deserialize_size_from_str")] pub size : u64 , pub preprocessor_cache_mode : PreprocessorCacheModeConfig , pub rw_mode : CacheModeConfig , }
};
}
