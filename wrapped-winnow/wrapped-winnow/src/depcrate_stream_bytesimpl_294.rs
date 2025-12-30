// Generated macro for impl_294 (impl)
macro_rules! Depcrate_stream_bytesimpl_294 {
() => {
// Module: crate::stream::bytes
// Provides: {"impl_294"}
// Dependencies: {}
impl ops :: Index < ops :: RangeToInclusive < usize > > for Bytes { type Output = Bytes ; # [inline] fn index (& self , r : ops :: RangeToInclusive < usize >) -> & Bytes { Bytes :: new (& self . as_bytes () [..= r . end]) } }
};
}
