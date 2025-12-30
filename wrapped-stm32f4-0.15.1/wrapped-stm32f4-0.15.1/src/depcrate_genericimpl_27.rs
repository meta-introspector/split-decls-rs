// Generated macro for impl_27 (impl)
macro_rules! Depcrate_genericimpl_27 {
() => {
// Module: crate::generic
// Provides: {"impl_27"}
// Dependencies: {}
impl < FI > PartialEq < FI > for BitReader < FI > where FI : Copy + Into < bool > , { # [inline (always)] fn eq (& self , other : & FI) -> bool { self . bits . eq (& (* other) . into ()) } }
};
}
