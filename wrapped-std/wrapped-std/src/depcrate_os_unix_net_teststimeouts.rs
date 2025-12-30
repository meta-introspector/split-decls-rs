// Generated macro for timeouts (function)
macro_rules! Depcrate_os_unix_net_teststimeouts {
() => {
// Module: crate::os::unix::net::tests
// Provides: {"timeouts"}
// Dependencies: {}
# [test] # [cfg (not (target_os = "nto"))] # [cfg_attr (target_os = "android" , ignore)] # [cfg_attr (target_os = "cygwin" , ignore)] fn timeouts () { let dir = tmpdir () ; let socket_path = dir . path () . join ("sock") ; let _listener = or_panic ! (UnixListener :: bind (& socket_path)) ; let stream = or_panic ! (UnixStream :: connect (& socket_path)) ; let dur = Duration :: new (15410 , 0) ; assert_eq ! (None , or_panic ! (stream . read_timeout ())) ; or_panic ! (stream . set_read_timeout (Some (dur))) ; assert_eq ! (Some (dur) , or_panic ! (stream . read_timeout ())) ; assert_eq ! (None , or_panic ! (stream . write_timeout ())) ; or_panic ! (stream . set_write_timeout (Some (dur))) ; assert_eq ! (Some (dur) , or_panic ! (stream . write_timeout ())) ; or_panic ! (stream . set_read_timeout (None)) ; assert_eq ! (None , or_panic ! (stream . read_timeout ())) ; or_panic ! (stream . set_write_timeout (None)) ; assert_eq ! (None , or_panic ! (stream . write_timeout ())) ; }
};
}
