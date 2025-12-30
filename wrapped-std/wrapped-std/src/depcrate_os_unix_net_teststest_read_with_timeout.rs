// Generated macro for test_read_with_timeout (function)
macro_rules! Depcrate_os_unix_net_teststest_read_with_timeout {
() => {
// Module: crate::os::unix::net::tests
// Provides: {"test_read_with_timeout"}
// Dependencies: {}
# [test] # [cfg_attr (target_os = "android" , ignore)] # [cfg_attr (target_os = "cygwin" , ignore)] fn test_read_with_timeout () { let dir = tmpdir () ; let socket_path = dir . path () . join ("sock") ; let listener = or_panic ! (UnixListener :: bind (& socket_path)) ; let mut stream = or_panic ! (UnixStream :: connect (& socket_path)) ; or_panic ! (stream . set_read_timeout (Some (Duration :: from_millis (1000)))) ; let mut other_end = or_panic ! (listener . accept ()) . 0 ; or_panic ! (other_end . write_all (b"hello world")) ; let mut buf = [0 ; 11] ; or_panic ! (stream . read (& mut buf)) ; assert_eq ! (b"hello world" , & buf [..]) ; let kind = stream . read_exact (& mut buf) . err () . expect ("expected error") . kind () ; assert ! (kind == ErrorKind :: WouldBlock || kind == ErrorKind :: TimedOut , "unexpected_error: {:?}" , kind) ; }
};
}
