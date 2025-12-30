// Generated macro for impl_461 (impl)
macro_rules! Depcrate_uintimpl_461 {
() => {
// Module: crate::uint
// Provides: {"impl_461"}
// Dependencies: {}
# [doc = " Comparing non-terimal bits, with both having bit `B0`."] # [doc = " These are `Equal`, so we propagate `SoFar`."] impl < Ul , Ur , SoFar > PrivateCmp < UInt < Ur , B0 > , SoFar > for UInt < Ul , B0 > where Ul : Unsigned , Ur : Unsigned , SoFar : Ord , Ul : PrivateCmp < Ur , SoFar > , { type Output = PrivateCmpOut < Ul , Ur , SoFar > ; # [inline] fn private_cmp (& self , rhs : & UInt < Ur , B0 > , so_far : SoFar) -> Self :: Output { self . msb . private_cmp (& rhs . msb , so_far) } }
};
}
