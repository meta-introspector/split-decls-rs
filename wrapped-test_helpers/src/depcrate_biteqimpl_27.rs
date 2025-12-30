// Generated macro for impl_27 (impl)
macro_rules! Depcrate_biteqimpl_27 {
() => {
// Module: crate::biteq
// Provides: {"impl_27"}
// Dependencies: {}
impl < T : BitEq > PartialEq < BitEqEitherWrapper < '_ , T > > for BitEqWrapper < '_ , T > { fn eq (& self , other : & BitEqEitherWrapper < '_ , T >) -> bool { self . 0 . biteq (other . 0) || self . 0 . biteq (other . 1) } }
};
}
