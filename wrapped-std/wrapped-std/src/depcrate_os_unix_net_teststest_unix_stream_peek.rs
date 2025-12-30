// Generated macro for test_unix_stream_peek (function)
macro_rules! Depcrate_os_unix_net_teststest_unix_stream_peek {
() => {
// Module: crate::os::unix::net::tests
// Provides: {"test_unix_stream_peek"}
// Dependencies: {}
# [test] # [cfg_attr (target_os = "android" , ignore)] fn test_unix_stream_peek () { let (txdone , rxdone) = crate :: sync :: mpsc :: channel () ; let dir = tmpdir () ; let path = dir . path () . join ("sock") ; let listener = or_panic ! (UnixListener :: bind (& path)) ; let thread = thread :: spawn (move | | { let mut stream = or_panic ! (listener . accept ()) . 0 ; or_panic ! (stream . write_all (& [1 , 3 , 3 , 7])) ; or_panic ! (rxdone . recv ()) ; }) ; let mut stream = or_panic ! (UnixStream :: connect (& path)) ; let mut buf = [0 ; 10] ; for _ in 0 .. 2 { assert_eq ! (or_panic ! (stream . peek (& mut buf)) , 4) ; } assert_eq ! (or_panic ! (stream . read (& mut buf)) , 4) ; or_panic ! (stream . set_nonblocking (true)) ; match stream . peek (& mut buf) { Ok (_) => panic ! ("expected error") , Err (ref e) if e . kind () == ErrorKind :: WouldBlock => { } Err (e) => panic ! ("unexpected error: {e}") , } or_panic ! (txdone . send (())) ; thread . join () . unwrap () ; }
};
}
