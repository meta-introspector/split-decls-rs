// Generated macro for impl_425 (impl)
macro_rules! Depcrate_uintimpl_425 {
() => {
// Module: crate::uint
// Provides: {"impl_425"}
// Dependencies: {}
# [doc = " Shifting right `UInt` by `UTerm`: `UInt<U, B> >> UTerm = UInt<U, B>`"] impl < U : Unsigned , B : Bit > Shr < UTerm > for UInt < U , B > { type Output = UInt < U , B > ; # [inline] fn shr (self , _ : UTerm) -> Self :: Output { UInt :: new () } }
};
}
