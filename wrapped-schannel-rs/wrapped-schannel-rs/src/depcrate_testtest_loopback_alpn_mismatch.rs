// Generated macro for test_loopback_alpn_mismatch (function)
macro_rules! Depcrate_testtest_loopback_alpn_mismatch {
() => {
// Module: crate::test
// Provides: {"test_loopback_alpn_mismatch"}
// Dependencies: {}
# [test] fn test_loopback_alpn_mismatch () { let cert = match localhost_cert () { Some (cert) => cert , None => return , } ; let listener = TcpListener :: bind ("127.0.0.1:0") . unwrap () ; let addr = listener . local_addr () . unwrap () ; let t = thread :: spawn (move | | { let stream = TcpStream :: connect (& addr) . unwrap () ; let creds = SchannelCred :: builder () . acquire (Direction :: Outbound) . unwrap () ; let mut stream = tls_stream :: Builder :: new () . domain ("localhost") . connect (creds , stream) . unwrap () ; assert_eq ! (stream . negotiated_application_protocol () . expect ("localhost unreachable") , None) ; stream . shutdown () . unwrap () ; }) ; let stream = listener . accept () . unwrap () . 0 ; let creds = SchannelCred :: builder () . cert (cert) . acquire (Direction :: Inbound) . unwrap () ; let stream = tls_stream :: Builder :: new () . request_application_protocols (& [b"h2"]) . accept (creds , stream) . unwrap () ; assert_eq ! (stream . negotiated_application_protocol () . expect ("localhost unreachable") , None) ; t . join () . unwrap () ; }
};
}
