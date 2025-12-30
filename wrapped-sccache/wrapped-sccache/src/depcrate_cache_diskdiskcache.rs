// Generated macro for DiskCache (struct)
macro_rules! Depcrate_cache_diskDiskCache {
() => {
// Module: crate::cache::disk
// Provides: {"DiskCache"}
// Dependencies: {}
# [doc = " A cache that stores entries at local disk paths."] pub struct DiskCache { # [doc = " `LruDiskCache` does all the real work here."] lru : Arc < Mutex < LazyDiskCache > > , # [doc = " Thread pool to execute disk I/O"] pool : tokio :: runtime :: Handle , preprocessor_cache_mode_config : PreprocessorCacheModeConfig , preprocessor_cache : Arc < Mutex < LazyDiskCache > > , rw_mode : CacheMode , }
};
}
