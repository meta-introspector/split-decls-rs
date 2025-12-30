// Generated macro for VerifiedPath (struct)
macro_rules! Depcrate_verify_certVerifiedPath {
() => {
// Module: crate::verify_cert
// Provides: {"VerifiedPath"}
// Dependencies: {}
# [doc = " Path from end-entity certificate to trust anchor that's been verified."] # [doc = ""] # [doc = " See [`EndEntityCert::verify_for_usage()`] for more details on what verification entails."] pub struct VerifiedPath < 'p > { end_entity : & 'p EndEntityCert < 'p > , intermediates : Intermediates < 'p > , anchor : & 'p TrustAnchor < 'p > , }
};
}
