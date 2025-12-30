// Generated macro for Verifier (struct)
macro_rules! Depcrate_verification_androidVerifier {
() => {
// Module: crate::verification::android
// Provides: {"Verifier"}
// Dependencies: {}
# [doc = " A TLS certificate verifier that utilizes the Android platform verifier."] # [derive (Debug)] pub struct Verifier { # [doc = " Testing only: The root CA certificate to trust."] # [cfg (any (test , feature = "ffi-testing"))] test_only_root_ca_override : Option < pki_types :: CertificateDer < 'static > > , crypto_provider : Arc < CryptoProvider > , }
};
}
