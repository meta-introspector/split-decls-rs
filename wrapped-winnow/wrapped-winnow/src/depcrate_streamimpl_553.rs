// Generated macro for impl_553 (impl)
macro_rules! Depcrate_streamimpl_553 {
() => {
// Module: crate::stream
// Provides: {"impl_553"}
// Dependencies: {}
impl < I , S > Offset for Checkpoint < I , S > where I : Offset , { # [inline (always)] fn offset_from (& self , start : & Self) -> usize { self . inner . offset_from (& start . inner) } }
};
}
