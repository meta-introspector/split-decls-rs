// Generated macro for validation_failure_is_permanent (function)
macro_rules! Depcrate_testvalidation_failure_is_permanent {
() => {
// Module: crate::test
// Provides: {"validation_failure_is_permanent"}
// Dependencies: {}
# [test] fn validation_failure_is_permanent () { let creds = SchannelCred :: builder () . acquire (Direction :: Outbound) . unwrap () ; let stream = TcpStream :: connect ("self-signed.badssl.com:443") . unwrap () ; stream . set_nonblocking (true) . unwrap () ; let stream = tls_stream :: Builder :: new () . domain ("self-signed.badssl.com") . connect (creds , stream) ; let stream = match stream { Err (HandshakeError :: Interrupted (s)) => s , _ => panic ! () , } ; stream . get_ref () . set_nonblocking (false) . unwrap () ; let err = unwrap_handshake (stream . handshake () . err () . unwrap ()) ; assert_eq ! (err . raw_os_error () . unwrap () , Foundation :: CERT_E_UNTRUSTEDROOT as i32) ; }
};
}
