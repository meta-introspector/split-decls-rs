// Generated macro for test_abstract_datagram_bind_send_to_addr (function)
macro_rules! Depcrate_os_unix_net_teststest_abstract_datagram_bind_send_to_addr {
() => {
// Module: crate::os::unix::net::tests
// Provides: {"test_abstract_datagram_bind_send_to_addr"}
// Dependencies: {}
# [cfg (any (target_os = "android" , target_os = "linux" , target_os = "cygwin"))] # [test] # [cfg_attr (target_os = "cygwin" , ignore)] fn test_abstract_datagram_bind_send_to_addr () { let addr1 = or_panic ! (SocketAddr :: from_abstract_name (b"ns1")) ; let sock1 = or_panic ! (UnixDatagram :: bind_addr (& addr1)) ; let local = or_panic ! (sock1 . local_addr ()) ; assert_eq ! (local . as_abstract_name () . unwrap () , b"ns1") ; let addr2 = or_panic ! (SocketAddr :: from_abstract_name (b"ns2")) ; let sock2 = or_panic ! (UnixDatagram :: bind_addr (& addr2)) ; let msg = b"hello world" ; or_panic ! (sock1 . send_to_addr (msg , & addr2)) ; let mut buf = [0 ; 11] ; let (len , addr) = or_panic ! (sock2 . recv_from (& mut buf)) ; assert_eq ! (msg , & buf [..]) ; assert_eq ! (len , 11) ; assert_eq ! (addr . as_abstract_name () . unwrap () , b"ns1") ; }
};
}
