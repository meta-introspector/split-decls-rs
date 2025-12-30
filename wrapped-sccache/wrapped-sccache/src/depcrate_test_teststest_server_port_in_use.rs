// Generated macro for test_server_port_in_use (function)
macro_rules! Depcrate_test_teststest_server_port_in_use {
() => {
// Module: crate::test::tests
// Provides: {"test_server_port_in_use"}
// Dependencies: {}
# [test] # [serial] # [cfg (not (target_os = "macos"))] fn test_server_port_in_use () { let listener = TcpListener :: bind ("127.0.0.1:0") . unwrap () ; let sccache = find_sccache_binary () ; let output = Command :: new (sccache) . arg ("--start-server") . env ("SCCACHE_SERVER_PORT" , listener . local_addr () . unwrap () . port () . to_string () ,) . env_remove ("SCCACHE_SERVER_UDS") . output () . unwrap () ; assert ! (! output . status . success ()) ; let s = String :: from_utf8_lossy (& output . stderr) ; const MSG : & str = "Server startup failed:" ; assert ! (s . contains (MSG) , "Output did not contain '{}':\n========\n{}\n========" , MSG , s) ; }
};
}
