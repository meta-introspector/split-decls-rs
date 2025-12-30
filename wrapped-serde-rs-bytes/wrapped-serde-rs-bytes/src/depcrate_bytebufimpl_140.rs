// Generated macro for impl_140 (impl)
macro_rules! Depcrate_bytebufimpl_140 {
() => {
// Module: crate::bytebuf
// Provides: {"impl_140"}
// Dependencies: {}
impl IntoIterator for ByteBuf { type Item = u8 ; type IntoIter = < Vec < u8 > as IntoIterator > :: IntoIter ; fn into_iter (self) -> Self :: IntoIter { self . bytes . into_iter () } }
};
}
