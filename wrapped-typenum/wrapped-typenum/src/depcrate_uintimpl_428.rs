// Generated macro for impl_428 (impl)
macro_rules! Depcrate_uintimpl_428 {
() => {
// Module: crate::uint
// Provides: {"impl_428"}
// Dependencies: {}
# [doc = " `UInt<Ul, B0> ^ UInt<Ur, B0> = UInt<Ul ^ Ur, B0>`"] impl < Ul : Unsigned , Ur : Unsigned > PrivateXor < UInt < Ur , B0 > > for UInt < Ul , B0 > where Ul : PrivateXor < Ur > , { type Output = UInt < PrivateXorOut < Ul , Ur > , B0 > ; # [inline] fn private_xor (self , rhs : UInt < Ur , B0 >) -> Self :: Output { UInt { msb : self . msb . private_xor (rhs . msb) , lsb : B0 , } } }
};
}
