// Generated macro for impl_393 (impl)
macro_rules! Depcrate_uintimpl_393 {
() => {
// Module: crate::uint
// Provides: {"impl_393"}
// Dependencies: {}
# [doc = " `UInt<Ul, B1> - UInt<Ur, B0> = UInt<Ul - Ur, B1>`"] impl < Ul : Unsigned , Ur : Unsigned > PrivateSub < UInt < Ur , B0 > > for UInt < Ul , B1 > where Ul : PrivateSub < Ur > , { type Output = UInt < PrivateSubOut < Ul , Ur > , B1 > ; # [inline] fn private_sub (self , rhs : UInt < Ur , B0 >) -> Self :: Output { UInt { msb : self . msb . private_sub (rhs . msb) , lsb : B1 , } } }
};
}
