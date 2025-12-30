// Generated macro for test_server_stats (function)
macro_rules! Depcrate_test_teststest_server_stats {
() => {
// Module: crate::test::tests
// Provides: {"test_server_stats"}
// Dependencies: {}
# [test] fn test_server_stats () { let f = TestFixture :: new () ; let (addr , sender , _storage , child) = run_server_thread (f . tempdir . path () , None) ; let conn = connect_to_server (& addr) . unwrap () ; let info = request_stats (conn) . unwrap () ; assert_eq ! (0 , info . stats . compile_requests) ; assert_eq ! (env ! ("CARGO_PKG_VERSION") , info . version) ; sender . send (ServerMessage :: Shutdown) . ok () . unwrap () ; child . join () . unwrap () ; }
};
}
