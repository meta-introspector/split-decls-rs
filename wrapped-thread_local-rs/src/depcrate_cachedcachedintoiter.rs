// Generated macro for CachedIntoIter (struct)
macro_rules! Depcrate_cachedCachedIntoIter {
() => {
// Module: crate::cached
// Provides: {"CachedIntoIter"}
// Dependencies: {}
# [doc = " An iterator that moves out of a `CachedThreadLocal`."] # [deprecated (since = "1.1.0" , note = "Use `IntoIter` instead")] pub struct CachedIntoIter < T : Send > { inner : IntoIter < T > , }
};
}
