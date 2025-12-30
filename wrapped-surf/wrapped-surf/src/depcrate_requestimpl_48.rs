// Generated macro for impl_48 (impl)
macro_rules! Depcrate_requestimpl_48 {
() => {
// Module: crate::request
// Provides: {"impl_48"}
// Dependencies: {}
impl IntoIterator for Request { type Item = (HeaderName , HeaderValues) ; type IntoIter = headers :: IntoIter ; # [doc = " Returns a iterator of references over the remaining items."] # [inline] fn into_iter (self) -> Self :: IntoIter { self . req . into_iter () } }
};
}
