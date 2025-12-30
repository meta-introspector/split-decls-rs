// Generated macro for no_session_resumed (function)
macro_rules! Depcrate_testno_session_resumed {
() => {
// Module: crate::test
// Provides: {"no_session_resumed"}
// Dependencies: {}
# [test] fn no_session_resumed () { for _ in 0 .. 2 { let creds = SchannelCred :: builder () . acquire (Direction :: Outbound) . unwrap () ; let stream = TcpStream :: connect ("google.com:443") . unwrap () ; let stream = tls_stream :: Builder :: new () . domain ("google.com") . connect (creds , stream) . unwrap () ; assert ! (! stream . session_resumed () . unwrap ()) ; } }
};
}
