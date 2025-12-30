// Generated macro for test_unix_datagram_peek (function)
macro_rules! Depcrate_os_unix_net_teststest_unix_datagram_peek {
() => {
// Module: crate::os::unix::net::tests
// Provides: {"test_unix_datagram_peek"}
// Dependencies: {}
# [test] # [cfg_attr (target_os = "android" , ignore)] fn test_unix_datagram_peek () { let dir = tmpdir () ; let path1 = dir . path () . join ("sock") ; let sock1 = or_panic ! (UnixDatagram :: bind (& path1)) ; let sock2 = or_panic ! (UnixDatagram :: unbound ()) ; or_panic ! (sock2 . connect (& path1)) ; let msg = b"hello world" ; or_panic ! (sock2 . send (msg)) ; for _ in 0 .. 2 { let mut buf = [0 ; 11] ; let size = or_panic ! (sock1 . peek (& mut buf)) ; assert_eq ! (size , 11) ; assert_eq ! (msg , & buf [..]) ; } let mut buf = [0 ; 11] ; let size = or_panic ! (sock1 . recv (& mut buf)) ; assert_eq ! (size , 11) ; assert_eq ! (msg , & buf [..]) ; }
};
}
