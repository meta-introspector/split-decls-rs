// Generated macro for verify_callback_success (function)
macro_rules! Depcrate_testverify_callback_success {
() => {
// Module: crate::test
// Provides: {"verify_callback_success"}
// Dependencies: {}
# [test] fn verify_callback_success () { let creds = SchannelCred :: builder () . acquire (Direction :: Outbound) . unwrap () ; let stream = TcpStream :: connect ("self-signed.badssl.com:443") . unwrap () ; let mut stream = tls_stream :: Builder :: new () . domain ("self-signed.badssl.com") . verify_callback (| validation_result | { assert ! (validation_result . result () . is_err ()) ; Ok (()) }) . connect (creds , stream) . unwrap () ; stream . write_all (b"GET / HTTP/1.0\r\nHost: self-signed.badssl.com\r\n\r\n") . unwrap () ; let mut out = vec ! [] ; stream . read_to_end (& mut out) . unwrap () ; assert ! (out . starts_with (b"HTTP/1.1 200 OK")) ; assert ! (out . ends_with (b"</html>\n")) ; }
};
}
