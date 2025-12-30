// Generated macro for CertificateIdentity (struct)
macro_rules! Depcrate_crypto_signerCertificateIdentity {
() => {
// Module: crate::crypto::signer
// Provides: {"CertificateIdentity"}
// Dependencies: {}
# [doc = " Data required to verify the peer's identity."] # [non_exhaustive] # [derive (Clone , Debug , Eq , PartialEq)] pub struct CertificateIdentity < 'a > { # [doc = " Certificate for the entity being verified."] pub end_entity : CertificateDer < 'a > , # [doc = " All certificates other than `end_entity` received in the peer's `Certificate` message."] # [doc = ""] # [doc = " It is in the same order that the peer sent them and may be empty."] pub intermediates : Vec < CertificateDer < 'a > > , }
};
}
