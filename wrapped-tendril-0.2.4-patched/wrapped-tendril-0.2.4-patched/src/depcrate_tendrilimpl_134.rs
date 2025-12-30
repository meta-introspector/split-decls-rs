// Generated macro for impl_134 (impl)
macro_rules! Depcrate_tendrilimpl_134 {
() => {
// Module: crate::tendril
// Provides: {"impl_134"}
// Dependencies: {}
impl < 'a , F , A > FromIterator < & 'a Tendril < F , A > > for Tendril < F , A > where F : fmt :: Format + 'a , A : Atomicity , { from_iter_method ! (&'a Tendril < F , A >) ; }
};
}
