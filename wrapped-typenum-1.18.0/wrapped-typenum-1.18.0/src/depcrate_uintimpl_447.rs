// Generated macro for impl_447 (impl)
macro_rules! Depcrate_uintimpl_447 {
() => {
// Module: crate::uint
// Provides: {"impl_447"}
// Dependencies: {}
# [doc = " Comparing non-terimal bits, with both having bit `B1`."] # [doc = " These are `Equal`, so we propagate `SoFar`."] impl < Ul , Ur , SoFar > PrivateCmp < UInt < Ur , B1 > , SoFar > for UInt < Ul , B1 > where Ul : Unsigned , Ur : Unsigned , SoFar : Ord , Ul : PrivateCmp < Ur , SoFar > , { type Output = PrivateCmpOut < Ul , Ur , SoFar > ; # [inline] fn private_cmp (& self , rhs : & UInt < Ur , B1 > , so_far : SoFar) -> Self :: Output { self . msb . private_cmp (& rhs . msb , so_far) } }
};
}
