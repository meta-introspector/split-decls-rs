// Generated macro for shutdown (function)
macro_rules! Depcrate_testshutdown {
() => {
// Module: crate::test
// Provides: {"shutdown"}
// Dependencies: {}
# [test] fn shutdown () { let creds = SchannelCred :: builder () . acquire (Direction :: Outbound) . unwrap () ; let stream = TcpStream :: connect ("google.com:443") . unwrap () ; let mut stream = tls_stream :: Builder :: new () . domain ("google.com") . connect (creds , stream) . unwrap () ; stream . shutdown () . unwrap () ; }
};
}
