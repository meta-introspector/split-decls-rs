// Generated macro for impl_227 (impl)
macro_rules! Depcrate_stream_bstrimpl_227 {
() => {
// Module: crate::stream::bstr
// Provides: {"impl_227"}
// Dependencies: {}
impl ops :: Index < ops :: RangeFrom < usize > > for BStr { type Output = BStr ; # [inline] fn index (& self , r : ops :: RangeFrom < usize >) -> & BStr { BStr :: new (& self . as_bytes () [r . start ..]) } }
};
}
