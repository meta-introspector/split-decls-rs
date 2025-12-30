// Generated macro for impl_41 (impl)
macro_rules! Depcrateimpl_41 {
() => {
// Module: crate
// Provides: {"impl_41"}
// Dependencies: {}
impl ClientConfigExt for rustls :: ConfigBuilder < ClientConfig , rustls :: WantsVerifier > { fn finish (self , kt : KeyType) -> ClientConfig { let mut root_store = RootCertStore :: empty () ; root_store . add_parsable_certificates (CertificateDer :: pem_slice_iter (kt . bytes_for ("ca.cert")) . map (| result | result . unwrap ()) ,) ; self . with_root_certificates (root_store) . with_no_client_auth () . unwrap () } fn finish_with_creds (self , kt : KeyType) -> ClientConfig { let mut root_store = RootCertStore :: empty () ; root_store . add_parsable_certificates (CertificateDer :: pem_slice_iter (kt . bytes_for ("ca.cert")) . map (| result | result . unwrap ()) ,) ; self . with_root_certificates (root_store) . with_client_auth_cert (kt . client_identity () , kt . client_key ()) . unwrap () } }
};
}
