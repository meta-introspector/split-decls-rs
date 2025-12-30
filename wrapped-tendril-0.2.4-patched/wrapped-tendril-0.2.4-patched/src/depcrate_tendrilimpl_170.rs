// Generated macro for impl_170 (impl)
macro_rules! Depcrate_tendrilimpl_170 {
() => {
// Module: crate::tendril
// Provides: {"impl_170"}
// Dependencies: {}
impl < 'a , A > From < & 'a Tendril < fmt :: UTF8 , A > > for String where A : Atomicity , { # [inline] fn from (input : & 'a Tendril < fmt :: UTF8 , A >) -> String { String :: from (& * * input) } }
};
}
