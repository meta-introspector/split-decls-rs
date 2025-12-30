// Generated macro for impl_65 (impl)
macro_rules! Depcrateimpl_65 {
() => {
// Module: crate
// Provides: {"impl_65"}
// Dependencies: {}
impl ServerVerifier for MockServerVerifier { fn verify_identity (& self , identity : & ServerIdentity < '_ >) -> Result < PeerVerified , Error > { println ! ("verify_identity({identity:?})") ; if let Some (expected_ocsp) = & self . expected_ocsp_response { assert_eq ! (expected_ocsp , identity . ocsp_response) ; } match & self . cert_rejection_error { Some (error) => Err (error . clone ()) , _ => Ok (PeerVerified :: assertion ()) , } } fn verify_tls12_signature (& self , input : & SignatureVerificationInput < '_ > ,) -> Result < HandshakeSignatureValid , Error > { println ! ("verify_tls12_signature({input:?})") ; match & self . tls12_signature_error { Some (error) => Err (error . clone ()) , _ => Ok (HandshakeSignatureValid :: assertion ()) , } } fn verify_tls13_signature (& self , input : & SignatureVerificationInput < '_ > ,) -> Result < HandshakeSignatureValid , Error > { println ! ("verify_tls13_signature({input:?})") ; match & self . tls13_signature_error { Some (error) => Err (error . clone ()) , _ if self . requires_raw_public_keys => verify_tls13_signature (input , self . raw_public_key_algorithms . as_ref () . unwrap () ,) , _ => Ok (HandshakeSignatureValid :: assertion ()) , } } fn supported_verify_schemes (& self) -> Vec < SignatureScheme > { self . signature_schemes . clone () } fn request_ocsp_response (& self) -> bool { self . expected_ocsp_response . is_some () } fn supported_certificate_types (& self) -> & 'static [CertificateType] { match self . requires_raw_public_keys { false => & [CertificateType :: X509] , true => & [CertificateType :: RawPublicKey] , } } }
};
}
