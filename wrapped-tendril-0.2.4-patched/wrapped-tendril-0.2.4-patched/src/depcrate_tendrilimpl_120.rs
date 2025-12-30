// Generated macro for impl_120 (impl)
macro_rules! Depcrate_tendrilimpl_120 {
() => {
// Module: crate::tendril
// Provides: {"impl_120"}
// Dependencies: {}
impl < F , A > Clone for Tendril < F , A > where F : fmt :: Format , A : Atomicity , { # [inline] fn clone (& self) -> Tendril < F , A > { unsafe { if self . ptr . get () . get () > MAX_INLINE_TAG { self . make_buf_shared () ; self . incref () ; } ptr :: read (self) } } }
};
}
