// Generated macro for test_unix_datagram_timeout_zero_duration (function)
macro_rules! Depcrate_os_unix_net_teststest_unix_datagram_timeout_zero_duration {
() => {
// Module: crate::os::unix::net::tests
// Provides: {"test_unix_datagram_timeout_zero_duration"}
// Dependencies: {}
# [test] # [cfg_attr (target_os = "android" , ignore)] fn test_unix_datagram_timeout_zero_duration () { let dir = tmpdir () ; let path = dir . path () . join ("sock") ; let datagram = or_panic ! (UnixDatagram :: bind (& path)) ; let result = datagram . set_write_timeout (Some (Duration :: new (0 , 0))) ; let err = result . unwrap_err () ; assert_eq ! (err . kind () , ErrorKind :: InvalidInput) ; let result = datagram . set_read_timeout (Some (Duration :: new (0 , 0))) ; let err = result . unwrap_err () ; assert_eq ! (err . kind () , ErrorKind :: InvalidInput) ; }
};
}
