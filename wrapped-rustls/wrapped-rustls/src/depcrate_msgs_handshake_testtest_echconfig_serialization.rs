// Generated macro for test_echconfig_serialization (function)
macro_rules! Depcrate_msgs_handshake_testtest_echconfig_serialization {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"test_echconfig_serialization"}
// Dependencies: {}
# [test] fn test_echconfig_serialization () { fn assert_round_trip_eq (original : & [u8]) { let configs = get_ech_config (original) ; let mut output = Vec :: new () ; configs . encode (& mut output) ; assert_eq ! (original , output) ; } assert_round_trip_eq (ECHCONFIG_LIST_LOCALHOST) ; assert_round_trip_eq (ECHCONFIG_LIST_CF) ; assert_round_trip_eq (ECHCONFIG_LIST_WITH_UNSUPPORTED) ; }
};
}
