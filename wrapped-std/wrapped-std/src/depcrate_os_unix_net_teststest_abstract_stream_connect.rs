// Generated macro for test_abstract_stream_connect (function)
macro_rules! Depcrate_os_unix_net_teststest_abstract_stream_connect {
() => {
// Module: crate::os::unix::net::tests
// Provides: {"test_abstract_stream_connect"}
// Dependencies: {}
# [cfg (any (target_os = "android" , target_os = "linux" , target_os = "cygwin"))] # [test] # [cfg_attr (target_os = "cygwin" , ignore)] fn test_abstract_stream_connect () { let msg1 = b"hello" ; let msg2 = b"world" ; let socket_addr = or_panic ! (SocketAddr :: from_abstract_name (b"name")) ; let listener = or_panic ! (UnixListener :: bind_addr (& socket_addr)) ; let thread = thread :: spawn (move | | { let mut stream = or_panic ! (listener . accept ()) . 0 ; let mut buf = [0 ; 5] ; or_panic ! (stream . read (& mut buf)) ; assert_eq ! (& msg1 [..] , & buf [..]) ; or_panic ! (stream . write_all (msg2)) ; }) ; let mut stream = or_panic ! (UnixStream :: connect_addr (& socket_addr)) ; let peer = or_panic ! (stream . peer_addr ()) ; assert_eq ! (peer . as_abstract_name () . unwrap () , b"name") ; or_panic ! (stream . write_all (msg1)) ; let mut buf = vec ! [] ; or_panic ! (stream . read_to_end (& mut buf)) ; assert_eq ! (& msg2 [..] , & buf [..]) ; drop (stream) ; thread . join () . unwrap () ; }
};
}
