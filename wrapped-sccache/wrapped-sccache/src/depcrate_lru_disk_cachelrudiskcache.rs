// Generated macro for LruDiskCache (struct)
macro_rules! Depcrate_lru_disk_cacheLruDiskCache {
() => {
// Module: crate::lru_disk_cache
// Provides: {"LruDiskCache"}
// Dependencies: {}
# [doc = " An LRU cache of files on disk."] pub struct LruDiskCache < S : BuildHasher = RandomState > { lru : LruCache < OsString , u64 , S , FileSize > , root : PathBuf , pending : Vec < OsString > , pending_size : u64 , }
};
}
