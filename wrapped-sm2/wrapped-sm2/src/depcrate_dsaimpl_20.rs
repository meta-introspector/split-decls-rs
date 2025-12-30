// Generated macro for impl_20 (impl)
macro_rules! Depcrate_dsaimpl_20 {
() => {
// Module: crate::dsa
// Provides: {"impl_20"}
// Dependencies: {}
# [cfg (feature = "arithmetic")] impl Signature { # [doc = " Get the `r` component of this signature"] pub fn r (& self) -> NonZeroScalar { NonZeroScalar :: new (self . r . into ()) . unwrap () } # [doc = " Get the `s` component of this signature"] pub fn s (& self) -> NonZeroScalar { NonZeroScalar :: new (self . s . into ()) . unwrap () } # [doc = " Split the signature into its `r` and `s` scalars."] pub fn split_scalars (& self) -> (NonZeroScalar , NonZeroScalar) { (self . r () , self . s ()) } }
};
}
