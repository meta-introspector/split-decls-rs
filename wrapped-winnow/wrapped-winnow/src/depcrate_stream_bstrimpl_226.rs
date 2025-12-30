// Generated macro for impl_226 (impl)
macro_rules! Depcrate_stream_bstrimpl_226 {
() => {
// Module: crate::stream::bstr
// Provides: {"impl_226"}
// Dependencies: {}
impl ops :: Index < ops :: RangeInclusive < usize > > for BStr { type Output = BStr ; # [inline] fn index (& self , r : ops :: RangeInclusive < usize >) -> & BStr { BStr :: new (& self . as_bytes () [* r . start () ..= * r . end ()]) } }
};
}
