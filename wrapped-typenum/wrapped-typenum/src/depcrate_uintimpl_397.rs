// Generated macro for impl_397 (impl)
macro_rules! Depcrate_uintimpl_397 {
() => {
// Module: crate::uint
// Provides: {"impl_397"}
// Dependencies: {}
# [doc = " `UInt<Ul, B1> + UInt<Ur, B1> = UInt<(Ul + Ur) + B1, B0>`"] impl < Ul : Unsigned , Ur : Unsigned > Add < UInt < Ur , B1 > > for UInt < Ul , B1 > where Ul : Add < Ur > , Sum < Ul , Ur > : Add < B1 > , { type Output = UInt < Add1 < Sum < Ul , Ur > > , B0 > ; # [inline] fn add (self , rhs : UInt < Ur , B1 >) -> Self :: Output { UInt { msb : self . msb + rhs . msb + B1 , lsb : B0 , } } }
};
}
