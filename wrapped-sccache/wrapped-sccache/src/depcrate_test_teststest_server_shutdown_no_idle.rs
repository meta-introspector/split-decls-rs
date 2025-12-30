// Generated macro for test_server_shutdown_no_idle (function)
macro_rules! Depcrate_test_teststest_server_shutdown_no_idle {
() => {
// Module: crate::test::tests
// Provides: {"test_server_shutdown_no_idle"}
// Dependencies: {}
# [doc = " The server will shutdown when requested when the idle timeout is disabled."] # [test] fn test_server_shutdown_no_idle () { let f = TestFixture :: new () ; let (addr , _sender , _storage , child) = run_server_thread (f . tempdir . path () , ServerOptions { idle_timeout : Some (0) , .. Default :: default () } ,) ; let conn = connect_to_server (& addr) . unwrap () ; request_shutdown (conn) . unwrap () ; child . join () . unwrap () ; }
};
}
