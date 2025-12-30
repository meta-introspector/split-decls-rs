// Generated macro for impl_292 (impl)
macro_rules! Depcrate_stream_bytesimpl_292 {
() => {
// Module: crate::stream::bytes
// Provides: {"impl_292"}
// Dependencies: {}
impl ops :: Index < ops :: RangeFrom < usize > > for Bytes { type Output = Bytes ; # [inline] fn index (& self , r : ops :: RangeFrom < usize >) -> & Bytes { Bytes :: new (& self . as_bytes () [r . start ..]) } }
};
}
