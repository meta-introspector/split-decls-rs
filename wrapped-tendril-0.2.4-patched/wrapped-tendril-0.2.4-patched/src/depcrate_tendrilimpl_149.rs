// Generated macro for impl_149 (impl)
macro_rules! Depcrate_tendrilimpl_149 {
() => {
// Module: crate::tendril
// Provides: {"impl_149"}
// Dependencies: {}
impl < F , A > From < Tendril < F , A > > for SendTendril < F > where F : fmt :: Format , A : Atomicity , { # [inline] fn from (tendril : Tendril < F , A >) -> SendTendril < F > { tendril . into_send () } }
};
}
