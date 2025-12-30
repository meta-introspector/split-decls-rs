// Generated macro for impl_70 (impl)
macro_rules! Depcrateimpl_70 {
() => {
// Module: crate
// Provides: {"impl_70"}
// Dependencies: {}
impl ClientVerifier for MockClientVerifier { fn verify_identity (& self , _identity : & ClientIdentity < '_ >) -> Result < PeerVerified , Error > { (self . verified) () } fn verify_tls12_signature (& self , input : & SignatureVerificationInput < '_ > ,) -> Result < HandshakeSignatureValid , Error > { if self . expect_raw_public_keys { Ok (HandshakeSignatureValid :: assertion ()) } else { self . parent . verify_tls12_signature (input) } } fn verify_tls13_signature (& self , input : & SignatureVerificationInput < '_ > ,) -> Result < HandshakeSignatureValid , Error > { if self . expect_raw_public_keys { verify_tls13_signature (input , self . raw_public_key_algorithms . as_ref () . unwrap () ,) } else { self . parent . verify_tls13_signature (input) } } fn root_hint_subjects (& self) -> Arc < [DistinguishedName] > { self . subjects . clone () } fn client_auth_mandatory (& self) -> bool { self . mandatory } fn supported_verify_schemes (& self) -> Vec < SignatureScheme > { if let Some (schemes) = & self . offered_schemes { schemes . clone () } else { self . parent . supported_verify_schemes () } } fn supported_certificate_types (& self) -> & 'static [CertificateType] { match self . expect_raw_public_keys { false => & [CertificateType :: X509] , true => & [CertificateType :: RawPublicKey] , } } }
};
}
