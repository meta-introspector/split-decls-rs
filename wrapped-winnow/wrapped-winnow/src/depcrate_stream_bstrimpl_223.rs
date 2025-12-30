// Generated macro for impl_223 (impl)
macro_rules! Depcrate_stream_bstrimpl_223 {
() => {
// Module: crate::stream::bstr
// Provides: {"impl_223"}
// Dependencies: {}
impl ops :: Index < usize > for BStr { type Output = u8 ; # [inline] fn index (& self , idx : usize) -> & u8 { & self . as_bytes () [idx] } }
};
}
