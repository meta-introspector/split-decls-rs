// Generated macro for SignerPublicKey (enum)
macro_rules! Depcrate_verifySignerPublicKey {
() => {
// Module: crate::verify
// Provides: {"SignerPublicKey"}
// Dependencies: {}
# [doc = " Public key used to verify a signature."] # [doc = ""] # [doc = " Used as part of [`SignatureVerificationInput`]."] # [non_exhaustive] # [derive (Debug)] pub enum SignerPublicKey < 'a > { # [doc = " An X.509 certificate for the signing peer."] X509 (& 'a CertificateDer < 'a >) , # [doc = " A raw public key, as defined in [RFC 7250](https://tools.ietf.org/html/rfc7250)."] RawPublicKey (& 'a SubjectPublicKeyInfoDer < 'a >) , }
};
}
