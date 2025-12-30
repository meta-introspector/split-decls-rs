// Generated macro for impl_270 (impl)
macro_rules! Depcrate_errorimpl_270 {
() => {
// Module: crate::error
// Provides: {"impl_270"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a Error { type Item = Error ; type IntoIter = Iter < 'a > ; fn into_iter (self) -> Self :: IntoIter { Iter { messages : self . messages . iter () , } } }
};
}
