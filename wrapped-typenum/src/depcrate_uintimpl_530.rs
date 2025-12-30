// Generated macro for impl_530 (impl)
macro_rules! Depcrate_uintimpl_530 {
() => {
// Module: crate::uint
// Provides: {"impl_530"}
// Dependencies: {}
impl < U , B , Ur > Min < Ur > for UInt < U , B > where U : Unsigned , B : Bit , Ur : Unsigned , UInt < U , B > : Cmp < Ur > + PrivateMin < Ur , Compare < UInt < U , B > , Ur > > , { type Output = PrivateMinOut < UInt < U , B > , Ur , Compare < UInt < U , B > , Ur > > ; # [inline] fn min (self , rhs : Ur) -> Self :: Output { self . private_min (rhs) } }
};
}
