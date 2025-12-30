// Generated macro for CachedIterMut (struct)
macro_rules! Depcrate_cachedCachedIterMut {
() => {
// Module: crate::cached
// Provides: {"CachedIterMut"}
// Dependencies: {}
# [doc = " Mutable iterator over the contents of a `CachedThreadLocal`."] # [deprecated (since = "1.1.0" , note = "Use `IterMut` instead")] pub struct CachedIterMut < 'a , T : Send + 'a > { inner : IterMut < 'a , T > , }
};
}
