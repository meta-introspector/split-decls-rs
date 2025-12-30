// Generated macro for impl_378 (impl)
macro_rules! Depcrate_stream_partialimpl_378 {
() => {
// Module: crate::stream::partial
// Provides: {"impl_378"}
// Dependencies: {}
impl < I > Offset for Partial < I > where I : Stream , { # [inline (always)] fn offset_from (& self , start : & Self) -> usize { self . offset_from (& start . checkpoint ()) } }
};
}
