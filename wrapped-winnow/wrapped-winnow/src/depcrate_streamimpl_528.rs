// Generated macro for impl_528 (impl)
macro_rules! Depcrate_streamimpl_528 {
() => {
// Module: crate::stream
// Provides: {"impl_528"}
// Dependencies: {}
impl < I > SliceLen for (I , usize , usize) where I : SliceLen , { # [inline (always)] fn slice_len (& self) -> usize { self . 0 . slice_len () * 8 + self . 2 - self . 1 } }
};
}
