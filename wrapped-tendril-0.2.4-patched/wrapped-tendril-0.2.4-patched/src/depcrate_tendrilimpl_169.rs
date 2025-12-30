// Generated macro for impl_169 (impl)
macro_rules! Depcrate_tendrilimpl_169 {
() => {
// Module: crate::tendril
// Provides: {"impl_169"}
// Dependencies: {}
impl < A > From < Tendril < fmt :: UTF8 , A > > for String where A : Atomicity , { # [inline] fn from (input : Tendril < fmt :: UTF8 , A >) -> String { String :: from (& * input) } }
};
}
