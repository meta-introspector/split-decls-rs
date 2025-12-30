// Generated macro for impl_402 (impl)
macro_rules! Depcrate_uintimpl_402 {
() => {
// Module: crate::uint
// Provides: {"impl_402"}
// Dependencies: {}
# [doc = " `UInt<Ul, B1> & UInt<Ur, B1> = UInt<Ul & Ur, B1>`"] impl < Ul : Unsigned , Ur : Unsigned > PrivateAnd < UInt < Ur , B1 > > for UInt < Ul , B1 > where Ul : PrivateAnd < Ur > , { type Output = UInt < PrivateAndOut < Ul , Ur > , B1 > ; # [inline] fn private_and (self , rhs : UInt < Ur , B1 >) -> Self :: Output { UInt { msb : self . msb . private_and (rhs . msb) , lsb : B1 , } } }
};
}
