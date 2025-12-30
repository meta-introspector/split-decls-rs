// Generated macro for ServerVerifierBuilder (struct)
macro_rules! Depcrate_webpki_server_verifierServerVerifierBuilder {
() => {
// Module: crate::webpki::server_verifier
// Provides: {"ServerVerifierBuilder"}
// Dependencies: {}
# [doc = " A builder for configuring a `webpki` server certificate verifier."] # [doc = ""] # [doc = " For more information, see the [`WebPkiServerVerifier`] documentation."] # [derive (Debug , Clone)] pub struct ServerVerifierBuilder { roots : Arc < RootCertStore > , crls : Vec < CertificateRevocationListDer < 'static > > , revocation_check_depth : RevocationCheckDepth , unknown_revocation_policy : UnknownStatusPolicy , revocation_expiration_policy : ExpirationPolicy , supported_algs : WebPkiSupportedAlgorithms , }
};
}
