// Generated macro for impl_9 (impl)
macro_rules! Depcrate_cachedimpl_9 {
() => {
// Module: crate::cached
// Provides: {"impl_9"}
// Dependencies: {}
impl < 'a , T : Send + 'a > IntoIterator for & 'a mut CachedThreadLocal < T > { type Item = & 'a mut T ; type IntoIter = CachedIterMut < 'a , T > ; fn into_iter (self) -> CachedIterMut < 'a , T > { self . iter_mut () } }
};
}
