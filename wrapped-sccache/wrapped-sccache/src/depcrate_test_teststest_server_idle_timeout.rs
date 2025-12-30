// Generated macro for test_server_idle_timeout (function)
macro_rules! Depcrate_test_teststest_server_idle_timeout {
() => {
// Module: crate::test::tests
// Provides: {"test_server_idle_timeout"}
// Dependencies: {}
# [test] fn test_server_idle_timeout () { let f = TestFixture :: new () ; let (_port , _sender , _storage , child) = run_server_thread (f . tempdir . path () , ServerOptions { idle_timeout : Some (1) , .. Default :: default () } ,) ; child . join () . unwrap () ; }
};
}
