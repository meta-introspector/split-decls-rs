// Generated macro for impl_168 (impl)
macro_rules! Depcrate_tendrilimpl_168 {
() => {
// Module: crate::tendril
// Provides: {"impl_168"}
// Dependencies: {}
impl < F , A > AsRef < F :: Slice > for Tendril < F , A > where F : fmt :: SliceFormat , A : Atomicity , { # [inline] fn as_ref (& self) -> & F :: Slice { & * * self } }
};
}
