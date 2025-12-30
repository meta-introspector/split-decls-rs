// Generated macro for tls_13 (function)
macro_rules! Depcrate_testtls_13 {
() => {
// Module: crate::test
// Provides: {"tls_13"}
// Dependencies: {}
# [test] fn tls_13 () { if env :: var ("SCHANNEL_SKIP_TLS_13_TEST") == Ok ("1" . to_owned ()) { return } let creds = SchannelCred :: builder () . enabled_protocols (& [Protocol :: Tls12 , Protocol :: Tls13]) . acquire (Direction :: Outbound) . unwrap () ; let stream = TcpStream :: connect ("tls13.akamai.io:443") . unwrap () ; let mut stream = tls_stream :: Builder :: new () . domain ("tls13.akamai.io") . connect (creds , stream) . unwrap () ; stream . write_all (b"GET / HTTP/1.0\r\nHost: tls13.akamai.io\r\n\r\n") . unwrap () ; let mut out = vec ! [] ; stream . read_to_end (& mut out) . unwrap () ; let pattern = b"Your client negotiated TLS 1.3" ; assert ! (out . windows (pattern . len ()) . any (| x | x == pattern)) ; }
};
}
