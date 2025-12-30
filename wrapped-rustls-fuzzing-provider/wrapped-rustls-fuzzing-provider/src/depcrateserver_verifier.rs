// Generated macro for server_verifier (function)
macro_rules! Depcrateserver_verifier {
() => {
// Module: crate
// Provides: {"server_verifier"}
// Dependencies: {}
pub fn server_verifier () -> Arc < dyn ServerVerifier > { let mut root_store = RootCertStore :: empty () ; root_store . add_parsable_certificates ([CertificateDer :: from (& include_bytes ! ("../../test-ca/ecdsa-p256/inter.der") [..] ,)]) ; WebPkiServerVerifier :: builder (root_store . into () , & PROVIDER) . build () . unwrap () }
};
}
