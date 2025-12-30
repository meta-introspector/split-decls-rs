// Generated macro for impl_497 (impl)
macro_rules! Depcrate_uintimpl_497 {
() => {
// Module: crate::uint
// Provides: {"impl_497"}
// Dependencies: {}
impl < Un , Bn , B > PrivateSetBit < U0 , B > for UInt < Un , Bn > { type Output = UInt < Un , B > ; # [inline] fn private_set_bit (self , _ : U0 , b : B) -> Self :: Output { UInt { msb : self . msb , lsb : b , } } }
};
}
