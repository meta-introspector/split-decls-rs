// Generated macro for verification_without_mock_root (function)
macro_rules! Depcrate_tests_verification_mockverification_without_mock_root {
() => {
// Module: crate::tests::verification_mock
// Provides: {"verification_without_mock_root"}
// Dependencies: {}
# [cfg (any (test , feature = "ffi-testing"))] # [cfg_attr (feature = "ffi-testing" , allow (dead_code))] pub (super) fn verification_without_mock_root () { let crypto_provider = test_provider () ; # [cfg (target_os = "freebsd")] let verifier = Verifier :: new_with_extra_roots (webpki_root_certs :: TLS_SERVER_ROOT_CERTS . iter () . cloned () , crypto_provider ,) . unwrap () ; # [cfg (not (target_os = "freebsd"))] let verifier = Verifier :: new (crypto_provider) . unwrap () ; let server_name = pki_types :: ServerName :: try_from (EXAMPLE_COM) . unwrap () ; let end_entity = pki_types :: CertificateDer :: from (ROOT1_INT1_EXAMPLE_COM_GOOD) ; let intermediates = [pki_types :: CertificateDer :: from (ROOT1_INT1)] ; let result = verifier . verify_server_cert (& end_entity , & intermediates , & server_name , & [] , verification_time () ,) ; assert_eq ! (result . map (| _ | ()) , Err (TlsError :: InvalidCertificate (CertificateError :: UnknownIssuer))) ; }
};
}
