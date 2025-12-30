// Generated macro for impl_142 (impl)
macro_rules! Depcrate_bytebufimpl_142 {
() => {
// Module: crate::bytebuf
// Provides: {"impl_142"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a mut ByteBuf { type Item = & 'a mut u8 ; type IntoIter = < & 'a mut [u8] as IntoIterator > :: IntoIter ; fn into_iter (self) -> Self :: IntoIter { self . bytes . iter_mut () } }
};
}
