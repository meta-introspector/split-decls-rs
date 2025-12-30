// Generated macro for impl_56 (impl)
macro_rules! Depcrate_verification_androidimpl_56 {
() => {
// Module: crate::verification::android
// Provides: {"impl_56"}
// Dependencies: {}
impl ServerCertVerifier for Verifier { fn verify_server_cert (& self , end_entity : & pki_types :: CertificateDer < '_ > , intermediates : & [pki_types :: CertificateDer < '_ >] , server_name : & pki_types :: ServerName , ocsp_response : & [u8] , now : pki_types :: UnixTime ,) -> Result < rustls :: client :: danger :: ServerCertVerified , TlsError > { log_server_cert (end_entity) ; let ocsp_data = if ! ocsp_response . is_empty () { Some (ocsp_response) } else { None } ; match self . verify_certificate (end_entity , intermediates , server_name , ocsp_data , now) { Ok (()) => Ok (rustls :: client :: danger :: ServerCertVerified :: assertion ()) , Err (e) => { log :: error ! ("failed to verify TLS certificate: {}" , e) ; Err (e) } } } fn verify_tls12_signature (& self , message : & [u8] , cert : & pki_types :: CertificateDer < '_ > , dss : & DigitallySignedStruct ,) -> Result < HandshakeSignatureValid , TlsError > { verify_tls12_signature (message , cert , dss , & self . crypto_provider . signature_verification_algorithms ,) } fn verify_tls13_signature (& self , message : & [u8] , cert : & pki_types :: CertificateDer < '_ > , dss : & DigitallySignedStruct ,) -> Result < HandshakeSignatureValid , TlsError > { verify_tls13_signature (message , cert , dss , & self . crypto_provider . signature_verification_algorithms ,) } fn supported_verify_schemes (& self) -> Vec < SignatureScheme > { self . crypto_provider . signature_verification_algorithms . supported_schemes () } }
};
}
