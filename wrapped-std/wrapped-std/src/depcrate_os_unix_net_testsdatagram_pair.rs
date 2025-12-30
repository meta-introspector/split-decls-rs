// Generated macro for datagram_pair (function)
macro_rules! Depcrate_os_unix_net_testsdatagram_pair {
() => {
// Module: crate::os::unix::net::tests
// Provides: {"datagram_pair"}
// Dependencies: {}
# [test] fn datagram_pair () { let msg1 = b"hello" ; let msg2 = b"world!" ; let (s1 , s2) = or_panic ! (UnixDatagram :: pair ()) ; let thread = thread :: spawn (move | | { let mut buf = [0 ; 5] ; or_panic ! (s1 . recv (& mut buf)) ; assert_eq ! (& msg1 [..] , & buf [..]) ; or_panic ! (s1 . send (msg2)) ; }) ; or_panic ! (s2 . send (msg1)) ; let mut buf = [0 ; 6] ; or_panic ! (s2 . recv (& mut buf)) ; assert_eq ! (& msg2 [..] , & buf [..]) ; drop (s2) ; thread . join () . unwrap () ; }
};
}
