// Generated macro for test_truncated_client_extension_is_detected (function)
macro_rules! Depcrate_msgs_handshake_testtest_truncated_client_extension_is_detected {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"test_truncated_client_extension_is_detected"}
// Dependencies: {}
# [test] fn test_truncated_client_extension_is_detected () { let chp = sample_client_hello_payload () ; let enc = chp . extensions . get_encoding () ; println ! ("testing enc {enc:?}") ; for l in 1 .. enc . len () { assert ! (ClientExtensions :: read_bytes (& enc [.. l]) . is_err ()) ; } }
};
}
