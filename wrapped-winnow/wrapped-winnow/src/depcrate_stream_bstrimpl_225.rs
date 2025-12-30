// Generated macro for impl_225 (impl)
macro_rules! Depcrate_stream_bstrimpl_225 {
() => {
// Module: crate::stream::bstr
// Provides: {"impl_225"}
// Dependencies: {}
impl ops :: Index < ops :: Range < usize > > for BStr { type Output = BStr ; # [inline] fn index (& self , r : ops :: Range < usize >) -> & BStr { BStr :: new (& self . as_bytes () [r . start .. r . end]) } }
};
}
