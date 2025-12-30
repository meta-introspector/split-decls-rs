// Generated macro for rejects_empty_sni_extension (function)
macro_rules! Depcrate_msgs_handshake_testrejects_empty_sni_extension {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"rejects_empty_sni_extension"}
// Dependencies: {}
# [test] fn rejects_empty_sni_extension () { assert_eq ! (ClientExtensions :: read_bytes (& [0 , 6 , 0 , 0 , 0 , 2 , 0 , 0]) . unwrap_err () , InvalidMessage :: IllegalEmptyList ("ServerNames")) ; }
};
}
