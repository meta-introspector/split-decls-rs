// Generated macro for CompressionCacheInner (struct)
macro_rules! Depcrate_compressCompressionCacheInner {
() => {
// Module: crate::compress
// Provides: {"CompressionCacheInner"}
// Dependencies: {}
# [doc = " Innards of an enabled CompressionCache."] # [doc = ""] # [doc = " You cannot make one of these directly. Use [`CompressionCache::new`]."] # [cfg (feature = "std")] # [derive (Debug)] pub struct CompressionCacheInner { # [doc = " Maximum size of underlying storage."] size : usize , # [doc = " LRU-order entries."] # [doc = ""] # [doc = " First is least-used, last is most-used."] entries : Mutex < VecDeque < Arc < CompressionCacheEntry > > > , }
};
}
