// Generated macro for ClientVerifierBuilder (struct)
macro_rules! Depcrate_webpki_client_verifierClientVerifierBuilder {
() => {
// Module: crate::webpki::client_verifier
// Provides: {"ClientVerifierBuilder"}
// Dependencies: {}
# [doc = " A builder for configuring a `webpki` client certificate verifier."] # [doc = ""] # [doc = " For more information, see the [`WebPkiClientVerifier`] documentation."] # [derive (Debug , Clone)] pub struct ClientVerifierBuilder { roots : Arc < RootCertStore > , root_hint_subjects : Vec < DistinguishedName > , crls : Vec < CertificateRevocationListDer < 'static > > , revocation_check_depth : RevocationCheckDepth , unknown_revocation_policy : UnknownStatusPolicy , revocation_expiration_policy : ExpirationPolicy , anon_policy : AnonymousClientPolicy , supported_algs : WebPkiSupportedAlgorithms , }
};
}
