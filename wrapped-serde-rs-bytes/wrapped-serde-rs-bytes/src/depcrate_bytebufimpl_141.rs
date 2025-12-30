// Generated macro for impl_141 (impl)
macro_rules! Depcrate_bytebufimpl_141 {
() => {
// Module: crate::bytebuf
// Provides: {"impl_141"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a ByteBuf { type Item = & 'a u8 ; type IntoIter = < & 'a [u8] as IntoIterator > :: IntoIter ; fn into_iter (self) -> Self :: IntoIter { self . bytes . iter () } }
};
}
