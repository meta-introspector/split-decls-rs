// Generated macro for impl_380 (impl)
macro_rules! Depcrate_uintimpl_380 {
() => {
// Module: crate::uint
// Provides: {"impl_380"}
// Dependencies: {}
# [doc = " `UInt<Ul, B0> + UInt<Ur, B1> = UInt<Ul + Ur, B1>`"] impl < Ul : Unsigned , Ur : Unsigned > Add < UInt < Ur , B1 > > for UInt < Ul , B0 > where Ul : Add < Ur > , { type Output = UInt < Sum < Ul , Ur > , B1 > ; # [inline] fn add (self , rhs : UInt < Ur , B1 >) -> Self :: Output { UInt { msb : self . msb + rhs . msb , lsb : B1 , } } }
};
}
