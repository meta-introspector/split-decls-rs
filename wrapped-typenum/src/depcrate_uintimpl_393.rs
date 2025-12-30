// Generated macro for impl_393 (impl)
macro_rules! Depcrate_uintimpl_393 {
() => {
// Module: crate::uint
// Provides: {"impl_393"}
// Dependencies: {}
# [doc = " `UInt<U, B> + UTerm = UInt<U, B>`"] impl < U : Unsigned , B : Bit > Add < UTerm > for UInt < U , B > { type Output = UInt < U , B > ; # [inline] fn add (self , _ : UTerm) -> Self :: Output { UInt :: new () } }
};
}
