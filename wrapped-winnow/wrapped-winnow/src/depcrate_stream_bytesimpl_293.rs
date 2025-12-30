// Generated macro for impl_293 (impl)
macro_rules! Depcrate_stream_bytesimpl_293 {
() => {
// Module: crate::stream::bytes
// Provides: {"impl_293"}
// Dependencies: {}
impl ops :: Index < ops :: RangeTo < usize > > for Bytes { type Output = Bytes ; # [inline] fn index (& self , r : ops :: RangeTo < usize >) -> & Bytes { Bytes :: new (& self . as_bytes () [.. r . end]) } }
};
}
