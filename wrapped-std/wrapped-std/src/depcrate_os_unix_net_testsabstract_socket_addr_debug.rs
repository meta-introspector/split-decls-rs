// Generated macro for abstract_socket_addr_debug (function)
macro_rules! Depcrate_os_unix_net_testsabstract_socket_addr_debug {
() => {
// Module: crate::os::unix::net::tests
// Provides: {"abstract_socket_addr_debug"}
// Dependencies: {}
# [cfg (any (target_os = "android" , target_os = "linux"))] # [test] fn abstract_socket_addr_debug () { assert_eq ! (r#""\0hello world\x01\x02\x03\x04\x05\x06\x07\x08\t\n\x11\x12\r\x14\x15\x16\x17\x18\x19\x1a\x1b\x1c\x1d\x1e\x1f \x7f\x80\x81\xfe\xff" (abstract)"# , format ! ("{:?}" , SocketAddr :: from_abstract_name (b"\0hello world\x01\x02\x03\x04\x05\x06\x07\x08\t\n\x11\x12\r\x14\x15\x16\x17\x18\x19\x1a\x1b\x1c\x1d\x1e\x1f \x7f\x80\x81\xfe\xff") . unwrap ()) ,) ; }
};
}
