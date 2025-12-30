// Generated macro for verify_callback_error (function)
macro_rules! Depcrate_testverify_callback_error {
() => {
// Module: crate::test
// Provides: {"verify_callback_error"}
// Dependencies: {}
# [test] fn verify_callback_error () { let creds = SchannelCred :: builder () . acquire (Direction :: Outbound) . unwrap () ; let stream = TcpStream :: connect ("google.com:443") . unwrap () ; let err = tls_stream :: Builder :: new () . domain ("google.com") . verify_callback (| validation_result | { assert ! (validation_result . result () . is_ok ()) ; Err (io :: Error :: from_raw_os_error (Foundation :: CERT_E_UNTRUSTEDROOT ,)) }) . connect (creds , stream) . err () . unwrap () ; let err = unwrap_handshake (err) ; assert_eq ! (err . raw_os_error () . unwrap () , Foundation :: CERT_E_UNTRUSTEDROOT as i32) ; }
};
}
