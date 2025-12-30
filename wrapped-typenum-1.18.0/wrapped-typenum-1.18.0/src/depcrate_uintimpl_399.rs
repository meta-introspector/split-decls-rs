// Generated macro for impl_399 (impl)
macro_rules! Depcrate_uintimpl_399 {
() => {
// Module: crate::uint
// Provides: {"impl_399"}
// Dependencies: {}
# [doc = " `UInt<Ul, B0> & UInt<Ur, B0> = UInt<Ul & Ur, B0>`"] impl < Ul : Unsigned , Ur : Unsigned > PrivateAnd < UInt < Ur , B0 > > for UInt < Ul , B0 > where Ul : PrivateAnd < Ur > , { type Output = UInt < PrivateAndOut < Ul , Ur > , B0 > ; # [inline] fn private_and (self , rhs : UInt < Ur , B0 >) -> Self :: Output { UInt { msb : self . msb . private_and (rhs . msb) , lsb : B0 , } } }
};
}
