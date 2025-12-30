// Generated macro for test_server_shutdown (function)
macro_rules! Depcrate_test_teststest_server_shutdown {
() => {
// Module: crate::test::tests
// Provides: {"test_server_shutdown"}
// Dependencies: {}
# [test] fn test_server_shutdown () { let f = TestFixture :: new () ; let (addr , _sender , _storage , child) = run_server_thread (f . tempdir . path () , None) ; let conn = connect_to_server (& addr) . unwrap () ; request_shutdown (conn) . unwrap () ; child . join () . unwrap () ; }
};
}
