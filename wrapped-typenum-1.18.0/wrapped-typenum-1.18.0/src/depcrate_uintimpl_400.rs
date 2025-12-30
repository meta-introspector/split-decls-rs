// Generated macro for impl_400 (impl)
macro_rules! Depcrate_uintimpl_400 {
() => {
// Module: crate::uint
// Provides: {"impl_400"}
// Dependencies: {}
# [doc = " `UInt<Ul, B0> & UInt<Ur, B1> = UInt<Ul & Ur, B0>`"] impl < Ul : Unsigned , Ur : Unsigned > PrivateAnd < UInt < Ur , B1 > > for UInt < Ul , B0 > where Ul : PrivateAnd < Ur > , { type Output = UInt < PrivateAndOut < Ul , Ur > , B0 > ; # [inline] fn private_and (self , rhs : UInt < Ur , B1 >) -> Self :: Output { UInt { msb : self . msb . private_and (rhs . msb) , lsb : B0 , } } }
};
}
