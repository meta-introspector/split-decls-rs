// Generated macro for impl_429 (impl)
macro_rules! Depcrate_uintimpl_429 {
() => {
// Module: crate::uint
// Provides: {"impl_429"}
// Dependencies: {}
# [doc = " Shifting right a `UInt` by a 1 bit: `UInt<U, B> >> B1 = U`"] impl < U : Unsigned , B : Bit > Shr < B1 > for UInt < U , B > { type Output = U ; # [inline] fn shr (self , _ : B1) -> Self :: Output { self . msb } }
};
}
