// Generated macro for expired_cert (function)
macro_rules! Depcrate_testexpired_cert {
() => {
// Module: crate::test
// Provides: {"expired_cert"}
// Dependencies: {}
# [test] fn expired_cert () { let creds = SchannelCred :: builder () . acquire (Direction :: Outbound) . unwrap () ; let stream = TcpStream :: connect ("expired.badssl.com:443") . unwrap () ; let err = tls_stream :: Builder :: new () . domain ("expired.badssl.com") . connect (creds , stream) . err () . unwrap () ; let err = unwrap_handshake (err) ; assert_eq ! (err . raw_os_error () . unwrap () , Foundation :: CERT_E_EXPIRED as i32) ; }
};
}
