// Generated macro for impl_64 (impl)
macro_rules! Depcrate_bytesimpl_64 {
() => {
// Module: crate::bytes
// Provides: {"impl_64"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a mut Bytes { type Item = & 'a mut u8 ; type IntoIter = < & 'a mut [u8] as IntoIterator > :: IntoIter ; fn into_iter (self) -> Self :: IntoIter { self . bytes . iter_mut () } }
};
}
