// Generated macro for impl_26 (impl)
macro_rules! Depcrate_genericimpl_26 {
() => {
// Module: crate::generic
// Provides: {"impl_26"}
// Dependencies: {}
impl < U , FI > PartialEq < FI > for FieldReader < U , FI > where U : PartialEq , FI : Copy + Into < U > , { # [inline (always)] fn eq (& self , other : & FI) -> bool { self . bits . eq (& (* other) . into ()) } }
};
}
