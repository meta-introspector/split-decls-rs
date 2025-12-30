// Generated macro for ChainOptions (struct)
macro_rules! Depcrate_verify_certChainOptions {
() => {
// Module: crate::verify_cert
// Provides: {"ChainOptions"}
// Dependencies: {}
pub (crate) struct ChainOptions < 'a , 'p > { pub (crate) eku : & 'a dyn ExtendedKeyUsageValidator , pub (crate) supported_sig_algs : & 'a [& 'a dyn SignatureVerificationAlgorithm] , pub (crate) trust_anchors : & 'p [TrustAnchor < 'p >] , pub (crate) intermediate_certs : & 'p [CertificateDer < 'p >] , pub (crate) revocation : Option < RevocationOptions < 'a > > , }
};
}
