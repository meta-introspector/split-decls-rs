// Generated macro for impl_394 (impl)
macro_rules! Depcrate_uintimpl_394 {
() => {
// Module: crate::uint
// Provides: {"impl_394"}
// Dependencies: {}
# [doc = " `UInt<Ul, B0> + UInt<Ur, B0> = UInt<Ul + Ur, B0>`"] impl < Ul : Unsigned , Ur : Unsigned > Add < UInt < Ur , B0 > > for UInt < Ul , B0 > where Ul : Add < Ur > , { type Output = UInt < Sum < Ul , Ur > , B0 > ; # [inline] fn add (self , rhs : UInt < Ur , B0 >) -> Self :: Output { UInt { msb : self . msb + rhs . msb , lsb : B0 , } } }
};
}
