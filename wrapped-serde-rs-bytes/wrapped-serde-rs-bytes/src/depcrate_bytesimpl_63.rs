// Generated macro for impl_63 (impl)
macro_rules! Depcrate_bytesimpl_63 {
() => {
// Module: crate::bytes
// Provides: {"impl_63"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a Bytes { type Item = & 'a u8 ; type IntoIter = < & 'a [u8] as IntoIterator > :: IntoIter ; fn into_iter (self) -> Self :: IntoIter { self . bytes . iter () } }
};
}
