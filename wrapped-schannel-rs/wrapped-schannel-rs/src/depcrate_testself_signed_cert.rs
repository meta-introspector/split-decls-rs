// Generated macro for self_signed_cert (function)
macro_rules! Depcrate_testself_signed_cert {
() => {
// Module: crate::test
// Provides: {"self_signed_cert"}
// Dependencies: {}
# [test] fn self_signed_cert () { let creds = SchannelCred :: builder () . acquire (Direction :: Outbound) . unwrap () ; let stream = TcpStream :: connect ("self-signed.badssl.com:443") . unwrap () ; let err = tls_stream :: Builder :: new () . domain ("self-signed.badssl.com") . connect (creds , stream) . err () . unwrap () ; let err = unwrap_handshake (err) ; assert_eq ! (err . raw_os_error () . unwrap () , Foundation :: CERT_E_UNTRUSTEDROOT as i32) ; }
};
}
