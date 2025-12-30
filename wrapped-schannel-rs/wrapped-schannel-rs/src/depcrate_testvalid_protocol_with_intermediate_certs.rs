// Generated macro for valid_protocol_with_intermediate_certs (function)
macro_rules! Depcrate_testvalid_protocol_with_intermediate_certs {
() => {
// Module: crate::test
// Provides: {"valid_protocol_with_intermediate_certs"}
// Dependencies: {}
# [test] fn valid_protocol_with_intermediate_certs () { let creds = SchannelCred :: builder () . enabled_protocols (& [Protocol :: Tls12]) . acquire (Direction :: Outbound) . unwrap () ; let stream = TcpStream :: connect ("lh3.googleusercontent.com:443") . unwrap () ; let mut stream = tls_stream :: Builder :: new () . domain ("lh3.googleusercontent.com") . connect (creds , stream) . unwrap () ; stream . write_all (b"GET / HTTP/1.0\r\n\r\n") . unwrap () ; let mut out = vec ! [] ; stream . read_to_end (& mut out) . unwrap () ; assert ! (out . starts_with (b"HTTP/1.0 200 OK") || out . starts_with (b"HTTP/1.0 302 Found")) ; assert ! (out . ends_with (b"</html>") || out . ends_with (b"</HTML>\r\n")) ; }
};
}
