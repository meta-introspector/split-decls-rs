// Generated macro for impl_241 (impl)
macro_rules! Depcrate_trust_anchorimpl_241 {
() => {
// Module: crate::trust_anchor
// Provides: {"impl_241"}
// Dependencies: {}
impl < 'a > From < Cert < 'a > > for TrustAnchor < 'a > { fn from (cert : Cert < 'a >) -> Self { Self { subject : cert . subject . as_slice_less_safe () . into () , subject_public_key_info : cert . spki . as_slice_less_safe () . into () , name_constraints : cert . name_constraints . map (| nc | nc . as_slice_less_safe () . into ()) , } } }
};
}
