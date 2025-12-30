// Generated macro for Identity (enum)
macro_rules! Depcrate_crypto_signerIdentity {
() => {
// Module: crate::crypto::signer
// Provides: {"Identity"}
// Dependencies: {}
# [doc = " A peer's identity, depending on the negotiated certificate type."] # [non_exhaustive] # [derive (Clone , Debug , Eq , PartialEq)] pub enum Identity < 'a > { # [doc = " A standard X.509 certificate chain."] # [doc = ""] # [doc = " This is the most common case."] X509 (CertificateIdentity < 'a >) , # [doc = " A raw public key, as defined in [RFC 7250](https://tools.ietf.org/html/rfc7250)."] RawPublicKey (SubjectPublicKeyInfoDer < 'a >) , }
};
}
