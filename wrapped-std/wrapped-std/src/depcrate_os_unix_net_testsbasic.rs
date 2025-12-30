// Generated macro for basic (function)
macro_rules! Depcrate_os_unix_net_testsbasic {
() => {
// Module: crate::os::unix::net::tests
// Provides: {"basic"}
// Dependencies: {}
# [test] # [cfg_attr (target_os = "android" , ignore)] fn basic () { let dir = tmpdir () ; let socket_path = dir . path () . join ("sock") ; let msg1 = b"hello" ; let msg2 = b"world!" ; let listener = or_panic ! (UnixListener :: bind (& socket_path)) ; let thread = thread :: spawn (move | | { let mut stream = or_panic ! (listener . accept ()) . 0 ; let mut buf = [0 ; 5] ; or_panic ! (stream . read (& mut buf)) ; assert_eq ! (& msg1 [..] , & buf [..]) ; or_panic ! (stream . write_all (msg2)) ; }) ; let mut stream = or_panic ! (UnixStream :: connect (& socket_path)) ; assert_eq ! (Some (&* socket_path) , stream . peer_addr () . unwrap () . as_pathname ()) ; or_panic ! (stream . write_all (msg1)) ; let mut buf = vec ! [] ; or_panic ! (stream . read_to_end (& mut buf)) ; assert_eq ! (& msg2 [..] , & buf [..]) ; drop (stream) ; thread . join () . unwrap () ; }
};
}
