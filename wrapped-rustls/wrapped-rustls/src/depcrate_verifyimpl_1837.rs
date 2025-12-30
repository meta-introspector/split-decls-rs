// Generated macro for impl_1837 (impl)
macro_rules! Depcrate_verifyimpl_1837 {
() => {
// Module: crate::verify
// Provides: {"impl_1837"}
// Dependencies: {}
impl ClientVerifier for NoClientAuth { fn verify_identity (& self , _identity : & ClientIdentity < '_ >) -> Result < PeerVerified , Error > { unimplemented ! () ; } fn verify_tls12_signature (& self , _input : & SignatureVerificationInput < '_ > ,) -> Result < HandshakeSignatureValid , Error > { unimplemented ! () ; } fn verify_tls13_signature (& self , _input : & SignatureVerificationInput < '_ > ,) -> Result < HandshakeSignatureValid , Error > { unimplemented ! () ; } fn root_hint_subjects (& self) -> Arc < [DistinguishedName] > { unimplemented ! () ; } fn offer_client_auth (& self) -> bool { false } fn supported_verify_schemes (& self) -> Vec < SignatureScheme > { unimplemented ! () ; } }
};
}
