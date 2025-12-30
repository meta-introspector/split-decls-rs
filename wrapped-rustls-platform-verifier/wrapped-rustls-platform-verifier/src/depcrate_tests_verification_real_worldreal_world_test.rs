// Generated macro for real_world_test (function)
macro_rules! Depcrate_tests_verification_real_worldreal_world_test {
() => {
// Module: crate::tests::verification_real_world
// Provides: {"real_world_test"}
// Dependencies: {}
fn real_world_test < E : std :: error :: Error > (test_case : & TestCase < E >) { log :: info ! ("verifying ref ID {:?} expected {:?}" , test_case . reference_id , test_case . expected_result) ; let crypto_provider = test_provider () ; # [cfg (target_os = "freebsd")] let verifier = Verifier :: new_with_extra_roots (webpki_root_certs :: TLS_SERVER_ROOT_CERTS . iter () . cloned () , crypto_provider ,) . unwrap () ; # [cfg (not (target_os = "freebsd"))] let verifier = Verifier :: new (crypto_provider) . unwrap () ; let mut chain = test_case . chain . iter () . map (| bytes | pki_types :: CertificateDer :: from (* bytes)) ; let end_entity_cert = chain . next () . unwrap () ; let intermediates : Vec < pki_types :: CertificateDer < '_ > > = chain . collect () ; let server_name = pki_types :: ServerName :: try_from (test_case . reference_id) . unwrap () ; let stapled_ocsp = test_case . stapled_ocsp . unwrap_or (& []) ; let result = verifier . verify_server_cert (& end_entity_cert , & intermediates , & server_name , stapled_ocsp , test_case . verification_time ,) . map (| _ | ()) ; assert_cert_error_eq (& result . map (| _ | ()) , & test_case . expected_result , None :: < & std :: convert :: Infallible > ,) ; }
};
}
