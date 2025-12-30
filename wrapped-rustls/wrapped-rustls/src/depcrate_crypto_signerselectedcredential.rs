// Generated macro for SelectedCredential (struct)
macro_rules! Depcrate_crypto_signerSelectedCredential {
() => {
// Module: crate::crypto::signer
// Provides: {"SelectedCredential"}
// Dependencies: {}
# [doc = " A packaged-together certificate chain and one-time-use signer."] # [doc = ""] # [doc = " This is used in the [`ClientCredentialResolver`] and [`ServerCredentialResolver`] traits"] # [doc = " as the return value of their `resolve()` methods."] # [non_exhaustive] # [derive (Debug)] pub struct SelectedCredential { # [doc = " The certificate chain or raw public key."] pub identity : Arc < Identity < 'static > > , # [doc = " The signing key matching the `identity`."] pub signer : Box < dyn Signer > , # [doc = " An optional OCSP response from the certificate issuer,"] # [doc = " attesting to its continued validity."] pub ocsp : Option < Arc < [u8] > > , }
};
}
