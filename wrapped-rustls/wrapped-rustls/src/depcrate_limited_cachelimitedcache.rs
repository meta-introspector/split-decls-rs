// Generated macro for LimitedCache (struct)
macro_rules! Depcrate_limited_cacheLimitedCache {
() => {
// Module: crate::limited_cache
// Provides: {"LimitedCache"}
// Dependencies: {}
# [doc = " A HashMap-alike, which never gets larger than a specified"] # [doc = " capacity, and evicts the oldest insertion to maintain this."] # [doc = ""] # [doc = " The requested capacity may be rounded up by the underlying"] # [doc = " collections.  This implementation uses all the allocated"] # [doc = " storage."] # [doc = ""] # [doc = " This is inefficient: it stores keys twice."] pub (crate) struct LimitedCache < K : Clone + Hash + Eq , V > { map : HashMap < K , V > , oldest : VecDeque < K > , }
};
}
