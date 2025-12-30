// Generated macro for test_unix_stream_timeout_zero_duration (function)
macro_rules! Depcrate_os_unix_net_teststest_unix_stream_timeout_zero_duration {
() => {
// Module: crate::os::unix::net::tests
// Provides: {"test_unix_stream_timeout_zero_duration"}
// Dependencies: {}
# [test] # [cfg_attr (target_os = "android" , ignore)] # [cfg_attr (target_os = "cygwin" , ignore)] fn test_unix_stream_timeout_zero_duration () { let dir = tmpdir () ; let socket_path = dir . path () . join ("sock") ; let listener = or_panic ! (UnixListener :: bind (& socket_path)) ; let stream = or_panic ! (UnixStream :: connect (& socket_path)) ; let result = stream . set_write_timeout (Some (Duration :: new (0 , 0))) ; let err = result . unwrap_err () ; assert_eq ! (err . kind () , ErrorKind :: InvalidInput) ; let result = stream . set_read_timeout (Some (Duration :: new (0 , 0))) ; let err = result . unwrap_err () ; assert_eq ! (err . kind () , ErrorKind :: InvalidInput) ; drop (listener) ; }
};
}
