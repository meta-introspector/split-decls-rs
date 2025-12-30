// Generated macro for test_truncated_client_hello_is_detected (function)
macro_rules! Depcrate_msgs_handshake_testtest_truncated_client_hello_is_detected {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"test_truncated_client_hello_is_detected"}
// Dependencies: {}
# [test] fn test_truncated_client_hello_is_detected () { let ch = sample_client_hello_payload () ; let enc = ch . get_encoding () ; println ! ("testing {ch:?} enc {enc:?}") ; for l in 0 .. enc . len () { println ! ("len {:?} enc {:?}" , l , & enc [.. l]) ; if l == 41 { continue ; } assert ! (ClientHelloPayload :: read_bytes (& enc [.. l]) . is_err ()) ; } }
};
}
