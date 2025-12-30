// Generated macro for impl_463 (impl)
macro_rules! Depcrate_uintimpl_463 {
() => {
// Module: crate::uint
// Provides: {"impl_463"}
// Dependencies: {}
# [doc = " Comparing non-terimal bits, with `Lhs` having bit `B0` and `Rhs` having bit `B1`."] # [doc = " `SoFar`, Lhs is `Less`."] impl < Ul , Ur , SoFar > PrivateCmp < UInt < Ur , B1 > , SoFar > for UInt < Ul , B0 > where Ul : Unsigned , Ur : Unsigned , SoFar : Ord , Ul : PrivateCmp < Ur , Less > , { type Output = PrivateCmpOut < Ul , Ur , Less > ; # [inline] fn private_cmp (& self , rhs : & UInt < Ur , B1 > , _ : SoFar) -> Self :: Output { self . msb . private_cmp (& rhs . msb , Less) } }
};
}
