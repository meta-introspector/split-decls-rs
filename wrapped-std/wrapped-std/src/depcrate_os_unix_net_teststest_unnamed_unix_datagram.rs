// Generated macro for test_unnamed_unix_datagram (function)
macro_rules! Depcrate_os_unix_net_teststest_unnamed_unix_datagram {
() => {
// Module: crate::os::unix::net::tests
// Provides: {"test_unnamed_unix_datagram"}
// Dependencies: {}
# [test] # [cfg_attr (target_os = "android" , ignore)] # [cfg_attr (target_os = "cygwin" , ignore)] fn test_unnamed_unix_datagram () { let dir = tmpdir () ; let path1 = dir . path () . join ("sock1") ; let sock1 = or_panic ! (UnixDatagram :: bind (& path1)) ; let sock2 = or_panic ! (UnixDatagram :: unbound ()) ; let msg = b"hello world" ; or_panic ! (sock2 . send_to (msg , & path1)) ; let mut buf = [0 ; 11] ; let (usize , addr) = or_panic ! (sock1 . recv_from (& mut buf)) ; assert_eq ! (usize , 11) ; assert ! (addr . is_unnamed ()) ; assert_eq ! (msg , & buf [..]) ; }
};
}
