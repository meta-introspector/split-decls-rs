// Generated macro for impl_17 (impl)
macro_rules! Depcrate_verification_othersimpl_17 {
() => {
// Module: crate::verification::others
// Provides: {"impl_17"}
// Dependencies: {}
impl ServerCertVerifier for Verifier { fn verify_server_cert (& self , end_entity : & pki_types :: CertificateDer < '_ > , intermediates : & [pki_types :: CertificateDer < '_ >] , server_name : & pki_types :: ServerName , ocsp_response : & [u8] , now : pki_types :: UnixTime ,) -> Result < ServerCertVerified , TlsError > { log_server_cert (end_entity) ; self . inner . verify_server_cert (end_entity , intermediates , server_name , ocsp_response , now) . map_err (map_webpki_errors) . map_err (| e | { log :: error ! ("failed to verify TLS certificate: {}" , e) ; e }) } fn verify_tls12_signature (& self , message : & [u8] , cert : & pki_types :: CertificateDer < '_ > , dss : & DigitallySignedStruct ,) -> Result < HandshakeSignatureValid , TlsError > { self . inner . verify_tls12_signature (message , cert , dss) } fn verify_tls13_signature (& self , message : & [u8] , cert : & pki_types :: CertificateDer < '_ > , dss : & DigitallySignedStruct ,) -> Result < HandshakeSignatureValid , TlsError > { self . inner . verify_tls13_signature (message , cert , dss) } fn supported_verify_schemes (& self) -> Vec < SignatureScheme > { self . inner . supported_verify_schemes () } }
};
}
