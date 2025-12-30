// Generated macro for impl_16 (impl)
macro_rules! Depcrate_genericimpl_16 {
() => {
// Module: crate::generic
// Provides: {"impl_16"}
// Dependencies: {}
impl < REG : RegisterSpec , FI > PartialEq < FI > for R < REG > where REG :: Ux : PartialEq , FI : Copy + Into < REG :: Ux > , { # [inline (always)] fn eq (& self , other : & FI) -> bool { self . bits . eq (& (* other) . into ()) } }
};
}
