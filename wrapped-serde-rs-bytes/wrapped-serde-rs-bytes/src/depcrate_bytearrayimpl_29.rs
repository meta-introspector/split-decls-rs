// Generated macro for impl_29 (impl)
macro_rules! Depcrate_bytearrayimpl_29 {
() => {
// Module: crate::bytearray
// Provides: {"impl_29"}
// Dependencies: {}
impl < 'a , const N : usize > IntoIterator for & 'a ByteArray < N > { type Item = & 'a u8 ; type IntoIter = < & 'a [u8 ; N] as IntoIterator > :: IntoIter ; fn into_iter (self) -> Self :: IntoIter { self . bytes . iter () } }
};
}
