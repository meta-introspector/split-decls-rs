// Generated macro for impl_49 (impl)
macro_rules! Depcrate_requestimpl_49 {
() => {
// Module: crate::request
// Provides: {"impl_49"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a Request { type Item = (& 'a HeaderName , & 'a HeaderValues) ; type IntoIter = headers :: Iter < 'a > ; # [inline] fn into_iter (self) -> Self :: IntoIter { self . req . iter () } }
};
}
