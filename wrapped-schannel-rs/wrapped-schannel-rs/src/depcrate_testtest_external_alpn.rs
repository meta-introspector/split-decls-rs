// Generated macro for test_external_alpn (function)
macro_rules! Depcrate_testtest_external_alpn {
() => {
// Module: crate::test
// Provides: {"test_external_alpn"}
// Dependencies: {}
# [test] fn test_external_alpn () { let creds = SchannelCred :: builder () . acquire (Direction :: Outbound) . unwrap () ; let stream = TcpStream :: connect ("google.com:443") . unwrap () ; let stream = tls_stream :: Builder :: new () . request_application_protocols (& [b"h2"]) . domain ("google.com") . connect (creds , stream) . unwrap () ; assert_eq ! (stream . negotiated_application_protocol () . expect ("google.com unreachable") , Some (b"h2" . to_vec ())) ; }
};
}
