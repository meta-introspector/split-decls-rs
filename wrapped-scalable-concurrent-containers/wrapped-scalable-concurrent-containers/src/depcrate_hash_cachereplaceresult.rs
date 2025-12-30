// Generated macro for ReplaceResult (enum)
macro_rules! Depcrate_hash_cacheReplaceResult {
() => {
// Module: crate::hash_cache
// Provides: {"ReplaceResult"}
// Dependencies: {}
# [doc = " [`ReplaceResult`] is the result type of the [`HashCache::replace_async`] and"] # [doc = " [`HashCache::replace_sync`] methods."] pub enum ReplaceResult < 'h , K , V , H = RandomState > where H : BuildHasher , { # [doc = " The key was replaced."] Replaced (OccupiedEntry < 'h , K , V , H > , K) , # [doc = " The key did not exist in the [`HashCache`]."] # [doc = ""] # [doc = " An [`OccupiedEntry`] can be created from the [`VacantEntry`]."] NotReplaced (VacantEntry < 'h , K , V , H >) , }
};
}
