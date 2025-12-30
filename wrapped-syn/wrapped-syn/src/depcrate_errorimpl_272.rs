// Generated macro for impl_272 (impl)
macro_rules! Depcrate_errorimpl_272 {
() => {
// Module: crate::error
// Provides: {"impl_272"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a Error { type Item = Error ; type IntoIter = Iter < 'a > ; fn into_iter (self) -> Self :: IntoIter { Iter { messages : self . messages . iter () , } } }
};
}
