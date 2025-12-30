// Generated macro for test_connect_unix_datagram (function)
macro_rules! Depcrate_os_unix_net_teststest_connect_unix_datagram {
() => {
// Module: crate::os::unix::net::tests
// Provides: {"test_connect_unix_datagram"}
// Dependencies: {}
# [test] # [cfg_attr (target_os = "android" , ignore)] # [cfg_attr (target_os = "cygwin" , ignore)] fn test_connect_unix_datagram () { let dir = tmpdir () ; let path1 = dir . path () . join ("sock1") ; let path2 = dir . path () . join ("sock2") ; let bsock1 = or_panic ! (UnixDatagram :: bind (& path1)) ; let bsock2 = or_panic ! (UnixDatagram :: bind (& path2)) ; let sock = or_panic ! (UnixDatagram :: unbound ()) ; or_panic ! (sock . connect (& path1)) ; let msg = b"hello there" ; or_panic ! (sock . send (msg)) ; let mut buf = [0 ; 11] ; let (usize , addr) = or_panic ! (bsock1 . recv_from (& mut buf)) ; assert_eq ! (usize , 11) ; assert ! (addr . is_unnamed ()) ; assert_eq ! (msg , & buf [..]) ; or_panic ! (sock . connect (& path2)) ; or_panic ! (sock . send (msg)) ; or_panic ! (bsock2 . recv_from (& mut buf)) ; }
};
}
