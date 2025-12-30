// Generated macro for impl_8 (impl)
macro_rules! Depcrate_cachedimpl_8 {
() => {
// Module: crate::cached
// Provides: {"impl_8"}
// Dependencies: {}
impl < T : Send > IntoIterator for CachedThreadLocal < T > { type Item = T ; type IntoIter = CachedIntoIter < T > ; fn into_iter (self) -> CachedIntoIter < T > { CachedIntoIter { inner : self . inner . into_iter () , } } }
};
}
