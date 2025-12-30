// Generated macro for SetBit (trait)
macro_rules! Depcrate_uintSetBit {
() => {
// Module: crate::uint
// Provides: {"SetBit"}
// Dependencies: {}
# [doc = " A **type operator** that, when implemented for unsigned integer `N`, sets the bit at position"] # [doc = " `I` to `B`."] pub trait SetBit < I , B > { # [allow (missing_docs)] type Output ; # [doc (hidden)] fn set_bit < IM : InternalMarker > (self , _ : I , _ : B) -> Self :: Output ; }
};
}
