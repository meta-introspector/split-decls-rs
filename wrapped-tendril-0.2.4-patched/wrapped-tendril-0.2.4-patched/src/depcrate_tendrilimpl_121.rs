// Generated macro for impl_121 (impl)
macro_rules! Depcrate_tendrilimpl_121 {
() => {
// Module: crate::tendril
// Provides: {"impl_121"}
// Dependencies: {}
impl < F , A > Drop for Tendril < F , A > where F : fmt :: Format , A : Atomicity , { # [inline] fn drop (& mut self) { unsafe { let p = self . ptr . get () . get () ; if p <= MAX_INLINE_TAG { return ; } let (buf , shared , _) = self . assume_buf () ; if shared { let header = self . header () ; if (* header) . refcount . decrement () == 1 { A :: fence_acquire () ; buf . destroy () ; } } else { buf . destroy () ; } } } }
};
}
