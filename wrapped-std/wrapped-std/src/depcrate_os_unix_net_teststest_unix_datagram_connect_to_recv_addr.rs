// Generated macro for test_unix_datagram_connect_to_recv_addr (function)
macro_rules! Depcrate_os_unix_net_teststest_unix_datagram_connect_to_recv_addr {
() => {
// Module: crate::os::unix::net::tests
// Provides: {"test_unix_datagram_connect_to_recv_addr"}
// Dependencies: {}
# [test] # [cfg_attr (target_os = "android" , ignore)] fn test_unix_datagram_connect_to_recv_addr () { let dir = tmpdir () ; let path1 = dir . path () . join ("sock1") ; let path2 = dir . path () . join ("sock2") ; let sock1 = or_panic ! (UnixDatagram :: bind (& path1)) ; let sock2 = or_panic ! (UnixDatagram :: bind (& path2)) ; let msg = b"hello world" ; let sock1_addr = or_panic ! (sock1 . local_addr ()) ; or_panic ! (sock2 . send_to_addr (msg , & sock1_addr)) ; let mut buf = [0 ; 11] ; let (_ , addr) = or_panic ! (sock1 . recv_from (& mut buf)) ; let new_msg = b"hello back" ; let mut new_buf = [0 ; 10] ; or_panic ! (sock2 . connect_addr (& addr)) ; or_panic ! (sock2 . send (new_msg)) ; let usize = or_panic ! (sock2 . recv (& mut new_buf)) ; assert_eq ! (usize , 10) ; assert_eq ! (new_msg , & new_buf [..]) ; }
};
}
