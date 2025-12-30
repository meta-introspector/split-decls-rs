// Generated macro for impl_291 (impl)
macro_rules! Depcrate_stream_bytesimpl_291 {
() => {
// Module: crate::stream::bytes
// Provides: {"impl_291"}
// Dependencies: {}
impl ops :: Index < ops :: RangeInclusive < usize > > for Bytes { type Output = Bytes ; # [inline] fn index (& self , r : ops :: RangeInclusive < usize >) -> & Bytes { Bytes :: new (& self . as_bytes () [* r . start () ..= * r . end ()]) } }
};
}
