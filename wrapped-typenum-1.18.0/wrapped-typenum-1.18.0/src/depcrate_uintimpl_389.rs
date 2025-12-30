// Generated macro for impl_389 (impl)
macro_rules! Depcrate_uintimpl_389 {
() => {
// Module: crate::uint
// Provides: {"impl_389"}
// Dependencies: {}
# [doc = " Subtracting unsigned integers. We just do our `PrivateSub` and then `Trim` the output."] impl < Ul : Unsigned , Bl : Bit , Ur : Unsigned > Sub < Ur > for UInt < Ul , Bl > where UInt < Ul , Bl > : PrivateSub < Ur > , PrivateSubOut < UInt < Ul , Bl > , Ur > : Trim , { type Output = TrimOut < PrivateSubOut < UInt < Ul , Bl > , Ur > > ; # [inline] fn sub (self , rhs : Ur) -> Self :: Output { self . private_sub (rhs) . trim () } }
};
}
