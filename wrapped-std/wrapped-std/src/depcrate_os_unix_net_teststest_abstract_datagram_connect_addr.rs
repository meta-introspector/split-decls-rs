// Generated macro for test_abstract_datagram_connect_addr (function)
macro_rules! Depcrate_os_unix_net_teststest_abstract_datagram_connect_addr {
() => {
// Module: crate::os::unix::net::tests
// Provides: {"test_abstract_datagram_connect_addr"}
// Dependencies: {}
# [cfg (any (target_os = "android" , target_os = "linux" , target_os = "cygwin"))] # [test] # [cfg_attr (target_os = "cygwin" , ignore)] fn test_abstract_datagram_connect_addr () { let addr1 = or_panic ! (SocketAddr :: from_abstract_name (b"ns3")) ; let bsock1 = or_panic ! (UnixDatagram :: bind_addr (& addr1)) ; let sock = or_panic ! (UnixDatagram :: unbound ()) ; or_panic ! (sock . connect_addr (& addr1)) ; let msg = b"hello world" ; or_panic ! (sock . send (msg)) ; let mut buf = [0 ; 11] ; let (len , addr) = or_panic ! (bsock1 . recv_from (& mut buf)) ; assert_eq ! (len , 11) ; assert_eq ! (addr . is_unnamed () , true) ; assert_eq ! (msg , & buf [..]) ; let addr2 = or_panic ! (SocketAddr :: from_abstract_name (b"ns4")) ; let bsock2 = or_panic ! (UnixDatagram :: bind_addr (& addr2)) ; or_panic ! (sock . connect_addr (& addr2)) ; or_panic ! (sock . send (msg)) ; or_panic ! (bsock2 . recv_from (& mut buf)) ; }
};
}
