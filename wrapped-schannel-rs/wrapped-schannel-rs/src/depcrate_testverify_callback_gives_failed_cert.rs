// Generated macro for verify_callback_gives_failed_cert (function)
macro_rules! Depcrate_testverify_callback_gives_failed_cert {
() => {
// Module: crate::test
// Provides: {"verify_callback_gives_failed_cert"}
// Dependencies: {}
# [test] fn verify_callback_gives_failed_cert () { let creds = SchannelCred :: builder () . acquire (Direction :: Outbound) . unwrap () ; let stream = TcpStream :: connect ("self-signed.badssl.com:443") . unwrap () ; let err = tls_stream :: Builder :: new () . domain ("self-signed.badssl.com") . verify_callback (| validation_result | { let expected_finger = include_bytes ! ("../test/self-signed.badssl.com.cer.sha1") . to_vec () ; assert_eq ! (validation_result . failed_certificate () . unwrap () . fingerprint (HashAlgorithm :: sha1 ()) . unwrap () , expected_finger) ; Err (io :: Error :: from_raw_os_error (Foundation :: CERT_E_UNTRUSTEDROOT ,)) }) . connect (creds , stream) . err () . unwrap () ; let err = unwrap_handshake (err) ; assert_eq ! (err . raw_os_error () . unwrap () , Foundation :: CERT_E_UNTRUSTEDROOT as i32) ; }
};
}
