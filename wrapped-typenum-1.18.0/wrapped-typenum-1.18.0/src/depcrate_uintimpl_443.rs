// Generated macro for impl_443 (impl)
macro_rules! Depcrate_uintimpl_443 {
() => {
// Module: crate::uint
// Provides: {"impl_443"}
// Dependencies: {}
# [doc = " `UInt<Ul, B1>` cmp with `UInt<Ur, B1>`: `SoFar` is `Equal`"] impl < Ul : Unsigned , Ur : Unsigned > Cmp < UInt < Ur , B1 > > for UInt < Ul , B1 > where Ul : PrivateCmp < Ur , Equal > , { type Output = PrivateCmpOut < Ul , Ur , Equal > ; # [inline] fn compare < IM : InternalMarker > (& self , rhs : & UInt < Ur , B1 >) -> Self :: Output { self . msb . private_cmp (& rhs . msb , Equal) } }
};
}
