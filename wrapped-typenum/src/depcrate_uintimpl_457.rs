// Generated macro for impl_457 (impl)
macro_rules! Depcrate_uintimpl_457 {
() => {
// Module: crate::uint
// Provides: {"impl_457"}
// Dependencies: {}
# [doc = " `UInt<Ul, B0>` cmp with `UInt<Ur, B0>`: `SoFar` is `Equal`"] impl < Ul : Unsigned , Ur : Unsigned > Cmp < UInt < Ur , B0 > > for UInt < Ul , B0 > where Ul : PrivateCmp < Ur , Equal > , { type Output = PrivateCmpOut < Ul , Ur , Equal > ; # [inline] fn compare < IM : InternalMarker > (& self , rhs : & UInt < Ur , B0 >) -> Self :: Output { self . msb . private_cmp (& rhs . msb , Equal) } }
};
}
