// Generated macro for test_unix_datagram (function)
macro_rules! Depcrate_os_unix_net_teststest_unix_datagram {
() => {
// Module: crate::os::unix::net::tests
// Provides: {"test_unix_datagram"}
// Dependencies: {}
# [test] # [cfg_attr (target_os = "android" , ignore)] fn test_unix_datagram () { let dir = tmpdir () ; let path1 = dir . path () . join ("sock1") ; let path2 = dir . path () . join ("sock2") ; let sock1 = or_panic ! (UnixDatagram :: bind (& path1)) ; let sock2 = or_panic ! (UnixDatagram :: bind (& path2)) ; let msg = b"hello world" ; or_panic ! (sock1 . send_to (msg , & path2)) ; let mut buf = [0 ; 11] ; or_panic ! (sock2 . recv_from (& mut buf)) ; assert_eq ! (msg , & buf [..]) ; }
};
}
