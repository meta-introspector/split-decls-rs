// Generated macro for impl_228 (impl)
macro_rules! Depcrate_stream_bstrimpl_228 {
() => {
// Module: crate::stream::bstr
// Provides: {"impl_228"}
// Dependencies: {}
impl ops :: Index < ops :: RangeTo < usize > > for BStr { type Output = BStr ; # [inline] fn index (& self , r : ops :: RangeTo < usize >) -> & BStr { BStr :: new (& self . as_bytes () [.. r . end]) } }
};
}
