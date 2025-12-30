// Generated macro for iter (function)
macro_rules! Depcrate_os_unix_net_testsiter {
() => {
// Module: crate::os::unix::net::tests
// Provides: {"iter"}
// Dependencies: {}
# [test] # [cfg_attr (target_os = "android" , ignore)] fn iter () { let dir = tmpdir () ; let socket_path = dir . path () . join ("sock") ; let listener = or_panic ! (UnixListener :: bind (& socket_path)) ; let thread = thread :: spawn (move | | { for stream in listener . incoming () . take (2) { let mut stream = or_panic ! (stream) ; let mut buf = [0] ; or_panic ! (stream . read (& mut buf)) ; } }) ; for _ in 0 .. 2 { let mut stream = or_panic ! (UnixStream :: connect (& socket_path)) ; or_panic ! (stream . write_all (& [0])) ; } thread . join () . unwrap () ; }
};
}
