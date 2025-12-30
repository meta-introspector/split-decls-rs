// Generated macro for verifier_for_dbg (function)
macro_rules! Depcrateverifier_for_dbg {
() => {
// Module: crate
// Provides: {"verifier_for_dbg"}
// Dependencies: {}
# [doc = " Exposed for debugging certificate issues with standalone tools."] # [doc = ""] # [doc = " This is not intended for production use, you should use [`BuilderVerifierExt`] or"] # [doc = " [`ConfigVerifierExt`] instead."] # [cfg (feature = "dbg")] pub fn verifier_for_dbg (root : CertificateDer < 'static > , crypto_provider : Arc < CryptoProvider > ,) -> Arc < dyn rustls :: client :: danger :: ServerCertVerifier > { Arc :: new (Verifier :: new_with_fake_root (root , crypto_provider)) }
};
}
