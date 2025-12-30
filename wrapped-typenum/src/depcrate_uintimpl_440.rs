// Generated macro for impl_440 (impl)
macro_rules! Depcrate_uintimpl_440 {
() => {
// Module: crate::uint
// Provides: {"impl_440"}
// Dependencies: {}
# [doc = " Shifting right `UInt` by `UTerm`: `UInt<U, B> >> UTerm = UInt<U, B>`"] impl < U : Unsigned , B : Bit > Shr < UTerm > for UInt < U , B > { type Output = UInt < U , B > ; # [inline] fn shr (self , _ : UTerm) -> Self :: Output { UInt :: new () } }
};
}
