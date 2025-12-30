// Generated macro for impl_522 (impl)
macro_rules! Depcrate_uintimpl_522 {
() => {
// Module: crate::uint
// Provides: {"impl_522"}
// Dependencies: {}
impl < U , B , Ur > Max < Ur > for UInt < U , B > where U : Unsigned , B : Bit , Ur : Unsigned , UInt < U , B > : Cmp < Ur > + PrivateMax < Ur , Compare < UInt < U , B > , Ur > > , { type Output = PrivateMaxOut < UInt < U , B > , Ur , Compare < UInt < U , B > , Ur > > ; # [inline] fn max (self , rhs : Ur) -> Self :: Output { self . private_max (rhs) } }
};
}
