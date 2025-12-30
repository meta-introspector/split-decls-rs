// Generated macro for impl_167 (impl)
macro_rules! Depcrate_tendrilimpl_167 {
() => {
// Module: crate::tendril
// Provides: {"impl_167"}
// Dependencies: {}
impl < A > From < String > for Tendril < fmt :: UTF8 , A > where A : Atomicity , { # [inline] fn from (input : String) -> Tendril < fmt :: UTF8 , A > { Tendril :: from_slice (& * input) } }
};
}
