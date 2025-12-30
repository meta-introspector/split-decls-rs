// Generated macro for CompressionCache (enum)
macro_rules! Depcrate_compressCompressionCache {
() => {
// Module: crate::compress
// Provides: {"CompressionCache"}
// Dependencies: {}
# [doc = " An LRU cache for compressions."] # [doc = ""] # [doc = " The prospect of being able to reuse a given compression for many connections"] # [doc = " means we can afford to spend more time on that compression (by passing"] # [doc = " `CompressionLevel::Amortized` to the compressor)."] # [expect (clippy :: exhaustive_enums)] # [derive (Debug)] pub enum CompressionCache { # [doc = " No caching happens, and compression happens each time using"] # [doc = " `CompressionLevel::Interactive`."] Disabled , # [doc = " Compressions are stored in an LRU cache."] # [cfg (feature = "std")] Enabled (CompressionCacheInner) , }
};
}
