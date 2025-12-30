// Generated macro for impl_150 (impl)
macro_rules! Depcrate_tendrilimpl_150 {
() => {
// Module: crate::tendril
// Provides: {"impl_150"}
// Dependencies: {}
impl < F , A > From < SendTendril < F > > for Tendril < F , A > where F : fmt :: Format , A : Atomicity , { # [inline] fn from (send : SendTendril < F >) -> Tendril < F , A > { unsafe { mem :: transmute (send . tendril) } } }
};
}
