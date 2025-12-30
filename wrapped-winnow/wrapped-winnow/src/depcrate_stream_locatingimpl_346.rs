// Generated macro for impl_346 (impl)
macro_rules! Depcrate_stream_locatingimpl_346 {
() => {
// Module: crate::stream::locating
// Provides: {"impl_346"}
// Dependencies: {}
impl < I > Offset for LocatingSlice < I > where I : Stream , { # [inline (always)] fn offset_from (& self , other : & Self) -> usize { self . offset_from (& other . checkpoint ()) } }
};
}
