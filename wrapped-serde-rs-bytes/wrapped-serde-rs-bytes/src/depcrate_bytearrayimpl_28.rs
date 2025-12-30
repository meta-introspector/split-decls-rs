// Generated macro for impl_28 (impl)
macro_rules! Depcrate_bytearrayimpl_28 {
() => {
// Module: crate::bytearray
// Provides: {"impl_28"}
// Dependencies: {}
impl < const N : usize > IntoIterator for ByteArray < N > { type Item = u8 ; type IntoIter = < [u8 ; N] as IntoIterator > :: IntoIter ; fn into_iter (self) -> Self :: IntoIter { IntoIterator :: into_iter (self . bytes) } }
};
}
