// Generated macro for LazyDiskCache (enum)
macro_rules! Depcrate_cache_diskLazyDiskCache {
() => {
// Module: crate::cache::disk
// Provides: {"LazyDiskCache"}
// Dependencies: {}
enum LazyDiskCache { Uninit { root : OsString , max_size : u64 } , Init (LruDiskCache) , }
};
}
