// Generated macro for test_abstract_stream_iter (function)
macro_rules! Depcrate_os_unix_net_teststest_abstract_stream_iter {
() => {
// Module: crate::os::unix::net::tests
// Provides: {"test_abstract_stream_iter"}
// Dependencies: {}
# [cfg (any (target_os = "android" , target_os = "linux" , target_os = "cygwin"))] # [test] # [cfg_attr (target_os = "cygwin" , ignore)] fn test_abstract_stream_iter () { let addr = or_panic ! (SocketAddr :: from_abstract_name (b"hidden")) ; let listener = or_panic ! (UnixListener :: bind_addr (& addr)) ; let thread = thread :: spawn (move | | { for stream in listener . incoming () . take (2) { let mut stream = or_panic ! (stream) ; let mut buf = [0] ; or_panic ! (stream . read (& mut buf)) ; } }) ; for _ in 0 .. 2 { let mut stream = or_panic ! (UnixStream :: connect_addr (& addr)) ; or_panic ! (stream . write_all (& [0])) ; } thread . join () . unwrap () ; }
};
}
