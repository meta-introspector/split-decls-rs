// Generated macro for try_clone (function)
macro_rules! Depcrate_os_unix_net_teststry_clone {
() => {
// Module: crate::os::unix::net::tests
// Provides: {"try_clone"}
// Dependencies: {}
# [test] # [cfg_attr (target_os = "android" , ignore)] fn try_clone () { let dir = tmpdir () ; let socket_path = dir . path () . join ("sock") ; let msg1 = b"hello" ; let msg2 = b"world" ; let listener = or_panic ! (UnixListener :: bind (& socket_path)) ; let thread = thread :: spawn (move | | { let mut stream = or_panic ! (listener . accept ()) . 0 ; or_panic ! (stream . write_all (msg1)) ; or_panic ! (stream . write_all (msg2)) ; }) ; let mut stream = or_panic ! (UnixStream :: connect (& socket_path)) ; let mut stream2 = or_panic ! (stream . try_clone ()) ; let mut buf = [0 ; 5] ; or_panic ! (stream . read (& mut buf)) ; assert_eq ! (& msg1 [..] , & buf [..]) ; or_panic ! (stream2 . read (& mut buf)) ; assert_eq ! (& msg2 [..] , & buf [..]) ; thread . join () . unwrap () ; }
};
}
