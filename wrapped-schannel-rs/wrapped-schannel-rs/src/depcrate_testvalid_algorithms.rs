// Generated macro for valid_algorithms (function)
macro_rules! Depcrate_testvalid_algorithms {
() => {
// Module: crate::test
// Provides: {"valid_algorithms"}
// Dependencies: {}
# [test] fn valid_algorithms () { let creds = SchannelCred :: builder () . supported_algorithms (& [Algorithm :: Aes128 , Algorithm :: Ecdsa]) . acquire (Direction :: Outbound) . unwrap () ; let stream = TcpStream :: connect ("google.com:443") . unwrap () ; let mut stream = tls_stream :: Builder :: new () . domain ("google.com") . connect (creds , stream) . unwrap () ; stream . write_all (b"GET / HTTP/1.0\r\n\r\n") . unwrap () ; let mut out = vec ! [] ; stream . read_to_end (& mut out) . unwrap () ; assert ! (out . starts_with (b"HTTP/1.0 200 OK") || out . starts_with (b"HTTP/1.0 302 Found")) ; assert ! (out . ends_with (b"</html>") || out . ends_with (b"</HTML>\r\n")) ; }
};
}
