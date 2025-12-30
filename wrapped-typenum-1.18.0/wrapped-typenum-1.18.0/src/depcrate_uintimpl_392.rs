// Generated macro for impl_392 (impl)
macro_rules! Depcrate_uintimpl_392 {
() => {
// Module: crate::uint
// Provides: {"impl_392"}
// Dependencies: {}
# [doc = " `UInt<Ul, B0> - UInt<Ur, B1> = UInt<(Ul - Ur) - B1, B1>`"] impl < Ul : Unsigned , Ur : Unsigned > PrivateSub < UInt < Ur , B1 > > for UInt < Ul , B0 > where Ul : PrivateSub < Ur > , PrivateSubOut < Ul , Ur > : Sub < B1 > , { type Output = UInt < Sub1 < PrivateSubOut < Ul , Ur > > , B1 > ; # [inline] fn private_sub (self , rhs : UInt < Ur , B1 >) -> Self :: Output { UInt { msb : self . msb . private_sub (rhs . msb) - B1 , lsb : B1 , } } }
};
}
