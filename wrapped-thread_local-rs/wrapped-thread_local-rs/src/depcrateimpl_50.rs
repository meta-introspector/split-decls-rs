// Generated macro for impl_50 (impl)
macro_rules! Depcrateimpl_50 {
() => {
// Module: crate
// Provides: {"impl_50"}
// Dependencies: {}
impl < 'a , T : Send + Sync > IntoIterator for & 'a ThreadLocal < T > { type Item = & 'a T ; type IntoIter = Iter < 'a , T > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
};
}
