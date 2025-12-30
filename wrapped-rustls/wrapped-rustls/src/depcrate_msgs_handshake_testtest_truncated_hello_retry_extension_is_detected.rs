// Generated macro for test_truncated_hello_retry_extension_is_detected (function)
macro_rules! Depcrate_msgs_handshake_testtest_truncated_hello_retry_extension_is_detected {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"test_truncated_hello_retry_extension_is_detected"}
// Dependencies: {}
# [test] fn test_truncated_hello_retry_extension_is_detected () { let hrr = sample_hello_retry_request () ; let mut enc = hrr . extensions . get_encoding () ; println ! ("testing enc {enc:?}") ; for l in 0 .. enc . len () { assert ! (HelloRetryRequestExtensions :: read_bytes (& enc [.. l]) . is_err ()) ; } for l in 0 .. (enc . len () - 4) { put_u16 (l as u16 , & mut enc) ; println ! ("  encoding {enc:?} len {l:?}") ; assert ! (HelloRetryRequestExtensions :: read_bytes (& enc) . is_err ()) ; } }
};
}
