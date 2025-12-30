// Generated macro for Verifier (struct)
macro_rules! Depcrate_verification_windowsVerifier {
() => {
// Module: crate::verification::windows
// Provides: {"Verifier"}
// Dependencies: {}
# [doc = " A TLS certificate verifier that utilizes the Windows certificate facilities."] # [derive (Debug)] pub struct Verifier { # [doc = " Testing only: The root CA certificate to trust."] # [cfg (any (test , feature = "ffi-testing" , feature = "dbg"))] test_only_root_ca_override : Option < pki_types :: CertificateDer < 'static > > , crypto_provider : Arc < CryptoProvider > , # [doc = " Extra trust anchors to add to the verifier above and beyond those provided by"] # [doc = " the system-provided trust stores."] extra_roots : Option < CertEngine > , }
};
}
