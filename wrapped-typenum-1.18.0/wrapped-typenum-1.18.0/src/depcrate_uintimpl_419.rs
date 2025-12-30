// Generated macro for impl_419 (impl)
macro_rules! Depcrate_uintimpl_419 {
() => {
// Module: crate::uint
// Provides: {"impl_419"}
// Dependencies: {}
# [doc = " Shifting left any unsigned by a zero bit: `U << B0 = U`"] impl < U : Unsigned , B : Bit > Shl < B0 > for UInt < U , B > { type Output = UInt < U , B > ; # [inline] fn shl (self , _ : B0) -> Self :: Output { UInt :: new () } }
};
}
