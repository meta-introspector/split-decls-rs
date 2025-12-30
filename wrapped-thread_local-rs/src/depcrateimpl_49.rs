// Generated macro for impl_49 (impl)
macro_rules! Depcrateimpl_49 {
() => {
// Module: crate
// Provides: {"impl_49"}
// Dependencies: {}
impl < T : Send > IntoIterator for ThreadLocal < T > { type Item = T ; type IntoIter = IntoIter < T > ; fn into_iter (self) -> IntoIter < T > { IntoIter { thread_local : self , raw : RawIter :: new () , } } }
};
}
