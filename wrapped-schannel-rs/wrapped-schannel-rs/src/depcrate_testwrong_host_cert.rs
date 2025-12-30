// Generated macro for wrong_host_cert (function)
macro_rules! Depcrate_testwrong_host_cert {
() => {
// Module: crate::test
// Provides: {"wrong_host_cert"}
// Dependencies: {}
# [test] fn wrong_host_cert () { let creds = SchannelCred :: builder () . acquire (Direction :: Outbound) . unwrap () ; let stream = TcpStream :: connect ("wrong.host.badssl.com:443") . unwrap () ; let err = tls_stream :: Builder :: new () . domain ("wrong.host.badssl.com") . connect (creds , stream) . err () . unwrap () ; let err = unwrap_handshake (err) ; assert_eq ! (err . raw_os_error () . unwrap () , Foundation :: CERT_E_CN_NO_MATCH as i32) ; }
};
}
