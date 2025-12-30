// Generated macro for impl_50 (impl)
macro_rules! Depcrate_requestimpl_50 {
() => {
// Module: crate::request
// Provides: {"impl_50"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a mut Request { type Item = (& 'a HeaderName , & 'a mut HeaderValues) ; type IntoIter = headers :: IterMut < 'a > ; # [inline] fn into_iter (self) -> Self :: IntoIter { self . req . iter_mut () } }
};
}
