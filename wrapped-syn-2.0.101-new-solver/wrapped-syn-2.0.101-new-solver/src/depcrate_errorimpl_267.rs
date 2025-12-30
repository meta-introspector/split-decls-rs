// Generated macro for impl_267 (impl)
macro_rules! Depcrate_errorimpl_267 {
() => {
// Module: crate::error
// Provides: {"impl_267"}
// Dependencies: {}
impl IntoIterator for Error { type Item = Error ; type IntoIter = IntoIter ; fn into_iter (self) -> Self :: IntoIter { IntoIter { messages : self . messages . into_iter () , } } }
};
}
