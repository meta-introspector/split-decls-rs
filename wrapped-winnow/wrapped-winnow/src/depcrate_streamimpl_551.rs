// Generated macro for impl_551 (impl)
macro_rules! Depcrate_streamimpl_551 {
() => {
// Module: crate::stream
// Provides: {"impl_551"}
// Dependencies: {}
impl < I > Offset for (I , usize) where I : Offset , { # [inline (always)] fn offset_from (& self , start : & Self) -> usize { self . 0 . offset_from (& start . 0) * 8 + self . 1 - start . 1 } }
};
}
