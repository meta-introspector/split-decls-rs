// Generated macro for impl_166 (impl)
macro_rules! Depcrate_tendrilimpl_166 {
() => {
// Module: crate::tendril
// Provides: {"impl_166"}
// Dependencies: {}
impl < 'a , F , A > From < & 'a F :: Slice > for Tendril < F , A > where F : fmt :: SliceFormat , A : Atomicity , { # [inline] fn from (input : & F :: Slice) -> Tendril < F , A > { Tendril :: from_slice (input) } }
};
}
