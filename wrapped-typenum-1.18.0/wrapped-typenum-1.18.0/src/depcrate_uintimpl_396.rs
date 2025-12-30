// Generated macro for impl_396 (impl)
macro_rules! Depcrate_uintimpl_396 {
() => {
// Module: crate::uint
// Provides: {"impl_396"}
// Dependencies: {}
# [doc = " Anding unsigned integers."] # [doc = " We use our `PrivateAnd` operator and then `Trim` the output."] impl < Ul : Unsigned , Bl : Bit , Ur : Unsigned > BitAnd < Ur > for UInt < Ul , Bl > where UInt < Ul , Bl > : PrivateAnd < Ur > , PrivateAndOut < UInt < Ul , Bl > , Ur > : Trim , { type Output = TrimOut < PrivateAndOut < UInt < Ul , Bl > , Ur > > ; # [inline] fn bitand (self , rhs : Ur) -> Self :: Output { self . private_and (rhs) . trim () } }
};
}
