// Generated macro for impl_30 (impl)
macro_rules! Depcrate_bytearrayimpl_30 {
() => {
// Module: crate::bytearray
// Provides: {"impl_30"}
// Dependencies: {}
impl < 'a , const N : usize > IntoIterator for & 'a mut ByteArray < N > { type Item = & 'a mut u8 ; type IntoIter = < & 'a mut [u8 ; N] as IntoIterator > :: IntoIter ; fn into_iter (self) -> Self :: IntoIter { self . bytes . iter_mut () } }
};
}
