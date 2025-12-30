// Generated macro for test_with_mock_root (function)
macro_rules! Depcrate_tests_verification_mocktest_with_mock_root {
() => {
// Module: crate::tests::verification_mock
// Provides: {"test_with_mock_root"}
// Dependencies: {}
fn test_with_mock_root < E : std :: error :: Error + PartialEq + 'static > (test_case : & TestCase < E > , root_src : Roots ,) { log :: info ! ("verifying {:?}" , test_case . expected_result) ; let provider = test_provider () ; let verifier = match root_src { Roots :: OnlyExtra => Verifier :: new_with_fake_root (ROOT1 , provider) , # [cfg (not (target_os = "android"))] Roots :: ExtraAndPlatform => Verifier :: new_with_extra_roots ([ROOT1] , provider) . unwrap () , } ; let mut chain = test_case . chain . iter () . map (| bytes | pki_types :: CertificateDer :: from (* bytes)) ; let end_entity = chain . next () . unwrap () ; let intermediates : Vec < pki_types :: CertificateDer < '_ > > = chain . collect () ; let server_name = pki_types :: ServerName :: try_from (test_case . reference_id) . unwrap () ; if test_case . reference_id . parse :: < IpAddr > () . is_ok () { assert ! (matches ! (server_name , pki_types :: ServerName :: IpAddress (_))) ; } else { assert ! (matches ! (server_name , pki_types :: ServerName :: DnsName (_))) ; } let result = verifier . verify_server_cert (& end_entity , & intermediates , & server_name , test_case . stapled_ocsp . unwrap_or (& []) , test_case . verification_time ,) ; assert_cert_error_eq (& result . map (| _ | ()) , & test_case . expected_result , test_case . other_error . as_ref () ,) ; }
};
}
