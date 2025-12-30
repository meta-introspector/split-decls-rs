// Generated macro for pair (function)
macro_rules! Depcrate_os_unix_net_testspair {
() => {
// Module: crate::os::unix::net::tests
// Provides: {"pair"}
// Dependencies: {}
# [test] fn pair () { let msg1 = b"hello" ; let msg2 = b"world!" ; let (mut s1 , mut s2) = or_panic ! (UnixStream :: pair ()) ; let thread = thread :: spawn (move | | { let mut buf = [0 ; 5] ; or_panic ! (s1 . read (& mut buf)) ; assert_eq ! (& msg1 [..] , & buf [..]) ; or_panic ! (s1 . write_all (msg2)) ; }) ; or_panic ! (s2 . write_all (msg1)) ; let mut buf = vec ! [] ; or_panic ! (s2 . read_to_end (& mut buf)) ; assert_eq ! (& msg2 [..] , & buf [..]) ; drop (s2) ; thread . join () . unwrap () ; }
};
}
