// Generated macro for impl_317 (impl)
macro_rules! Depcrate_verify_certimpl_317 {
() => {
// Module: crate::verify_cert
// Provides: {"impl_317"}
// Dependencies: {}
impl < 'p > VerifiedPath < 'p > { fn new (end_entity : & 'p EndEntityCert < 'p > , anchor : & 'p TrustAnchor < 'p > , partial : PartialPath < 'p > ,) -> Self { Self { end_entity , intermediates : Intermediates :: Owned { certs : partial . intermediates , used : partial . used , } , anchor , } } # [doc = " Yields a (double-ended) iterator over the intermediate certificates in this path."] pub fn intermediate_certificates (& 'p self) -> IntermediateIterator < 'p > { IntermediateIterator { intermediates : self . intermediates . as_ref () , } } # [doc = " Yields the end-entity certificate for this path."] pub fn end_entity (& self) -> & 'p EndEntityCert < 'p > { self . end_entity } # [doc = " Yields the trust anchor for this path."] pub fn anchor (& self) -> & 'p TrustAnchor < 'p > { self . anchor } }
};
}
