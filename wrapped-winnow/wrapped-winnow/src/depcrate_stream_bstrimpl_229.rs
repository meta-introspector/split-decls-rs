// Generated macro for impl_229 (impl)
macro_rules! Depcrate_stream_bstrimpl_229 {
() => {
// Module: crate::stream::bstr
// Provides: {"impl_229"}
// Dependencies: {}
impl ops :: Index < ops :: RangeToInclusive < usize > > for BStr { type Output = BStr ; # [inline] fn index (& self , r : ops :: RangeToInclusive < usize >) -> & BStr { BStr :: new (& self . as_bytes () [..= r . end]) } }
};
}
