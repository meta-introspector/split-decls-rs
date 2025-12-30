// Generated macro for Verifier (struct)
macro_rules! Depcrate_verification_appleVerifier {
() => {
// Module: crate::verification::apple
// Provides: {"Verifier"}
// Dependencies: {}
# [doc = " A TLS certificate verifier that utilizes the Apple platform certificate facilities."] # [derive (Debug)] pub struct Verifier { # [doc = " Extra trust anchors to add to the verifier above and beyond those provided by"] # [doc = " the system-provided trust stores."] extra_roots : Vec < SecCertificate > , # [doc = " Testing only: The root CA certificate to trust."] # [cfg (any (test , feature = "ffi-testing" , feature = "dbg"))] test_only_root_ca_override : Option < SecCertificate > , crypto_provider : Arc < CryptoProvider > , }
};
}
