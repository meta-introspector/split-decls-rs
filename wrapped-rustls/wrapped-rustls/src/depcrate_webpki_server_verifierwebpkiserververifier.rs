// Generated macro for WebPkiServerVerifier (struct)
macro_rules! Depcrate_webpki_server_verifierWebPkiServerVerifier {
() => {
// Module: crate::webpki::server_verifier
// Provides: {"WebPkiServerVerifier"}
// Dependencies: {}
# [doc = " Default `ServerVerifier`, see the trait impl for more information."] # [derive (Debug)] pub struct WebPkiServerVerifier { roots : Arc < RootCertStore > , crls : Vec < CertRevocationList < 'static > > , revocation_check_depth : RevocationCheckDepth , unknown_revocation_policy : UnknownStatusPolicy , revocation_expiration_policy : ExpirationPolicy , supported : WebPkiSupportedAlgorithms , }
};
}
