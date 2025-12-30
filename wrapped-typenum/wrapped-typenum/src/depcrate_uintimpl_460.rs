// Generated macro for impl_460 (impl)
macro_rules! Depcrate_uintimpl_460 {
() => {
// Module: crate::uint
// Provides: {"impl_460"}
// Dependencies: {}
# [doc = " `UInt<Ul, B1>` cmp with `UInt<Ur, B0>`: `SoFar` is `Greater`"] impl < Ul : Unsigned , Ur : Unsigned > Cmp < UInt < Ur , B0 > > for UInt < Ul , B1 > where Ul : PrivateCmp < Ur , Greater > , { type Output = PrivateCmpOut < Ul , Ur , Greater > ; # [inline] fn compare < IM : InternalMarker > (& self , rhs : & UInt < Ur , B0 >) -> Self :: Output { self . msb . private_cmp (& rhs . msb , Greater) } }
};
}
