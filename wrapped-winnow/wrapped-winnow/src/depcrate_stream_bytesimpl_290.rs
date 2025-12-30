// Generated macro for impl_290 (impl)
macro_rules! Depcrate_stream_bytesimpl_290 {
() => {
// Module: crate::stream::bytes
// Provides: {"impl_290"}
// Dependencies: {}
impl ops :: Index < ops :: Range < usize > > for Bytes { type Output = Bytes ; # [inline] fn index (& self , r : ops :: Range < usize >) -> & Bytes { Bytes :: new (& self . as_bytes () [r . start .. r . end]) } }
};
}
