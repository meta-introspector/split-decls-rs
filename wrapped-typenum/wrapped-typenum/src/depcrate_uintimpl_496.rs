// Generated macro for impl_496 (impl)
macro_rules! Depcrate_uintimpl_496 {
() => {
// Module: crate::uint
// Provides: {"impl_496"}
// Dependencies: {}
impl < N , I , B > SetBit < I , B > for N where N : PrivateSetBit < I , B > , PrivateSetBitOut < N , I , B > : Trim , { type Output = TrimOut < PrivateSetBitOut < N , I , B > > ; # [inline] fn set_bit < IM : InternalMarker > (self , i : I , b : B) -> Self :: Output { self . private_set_bit (i , b) . trim () } }
};
}
