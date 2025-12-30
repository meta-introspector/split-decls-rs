// Generated macro for basic_session_resumed (function)
macro_rules! Depcrate_testbasic_session_resumed {
() => {
// Module: crate::test
// Provides: {"basic_session_resumed"}
// Dependencies: {}
# [test] fn basic_session_resumed () { let creds = SchannelCred :: builder () . enabled_protocols (& [Protocol :: Tls12]) . acquire (Direction :: Outbound) . unwrap () ; let creds_copy = creds . clone () ; let stream = TcpStream :: connect ("google.com:443") . unwrap () ; let stream = tls_stream :: Builder :: new () . domain ("google.com") . connect (creds_copy , stream) . unwrap () ; assert ! (! stream . session_resumed () . unwrap ()) ; let stream = TcpStream :: connect ("google.com:443") . unwrap () ; let stream = tls_stream :: Builder :: new () . domain ("google.com") . connect (creds , stream) . unwrap () ; assert ! (stream . session_resumed () . unwrap ()) ; }
};
}
