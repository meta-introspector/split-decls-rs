// Generated macro for session_resumption_thread_safety (function)
macro_rules! Depcrate_testsession_resumption_thread_safety {
() => {
// Module: crate::test
// Provides: {"session_resumption_thread_safety"}
// Dependencies: {}
# [test] fn session_resumption_thread_safety () { let creds = SchannelCred :: builder () . enabled_protocols (& [Protocol :: Tls12]) . acquire (Direction :: Outbound) . unwrap () ; let creds_copy = creds . clone () ; let stream = TcpStream :: connect ("google.com:443") . unwrap () ; let stream = tls_stream :: Builder :: new () . domain ("google.com") . connect (creds_copy , stream) . unwrap () ; assert ! (! stream . session_resumed () . unwrap ()) ; let mut threads = vec ! [] ; for _ in 0 .. 4 { let creds_copy = creds . clone () ; threads . push (thread :: spawn (move | | { for _ in 0 .. 10 { let creds = creds_copy . clone () ; let stream = TcpStream :: connect ("google.com:443") . unwrap () ; let stream = tls_stream :: Builder :: new () . domain ("google.com") . connect (creds , stream) . unwrap () ; assert ! (stream . session_resumed () . unwrap ()) ; } })) ; } for thread in threads . into_iter () { thread . join () . unwrap () } }
};
}
