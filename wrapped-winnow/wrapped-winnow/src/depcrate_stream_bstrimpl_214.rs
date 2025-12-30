// Generated macro for impl_214 (impl)
macro_rules! Depcrate_stream_bstrimpl_214 {
() => {
// Module: crate::stream::bstr
// Provides: {"impl_214"}
// Dependencies: {}
impl Offset for & BStr { # [inline (always)] fn offset_from (& self , start : & Self) -> usize { self . as_bytes () . offset_from (& start . as_bytes ()) } }
};
}
