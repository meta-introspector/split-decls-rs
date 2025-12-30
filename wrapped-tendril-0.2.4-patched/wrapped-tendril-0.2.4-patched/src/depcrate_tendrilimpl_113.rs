// Generated macro for impl_113 (impl)
macro_rules! Depcrate_tendrilimpl_113 {
() => {
// Module: crate::tendril
// Provides: {"impl_113"}
// Dependencies: {}
impl < A > Header < A > where A : Atomicity , { # [inline (always)] unsafe fn new () -> Header < A > { Header { refcount : A :: new () , cap : mem :: uninitialized () , } } }
};
}
