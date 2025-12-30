// Generated macro for test_truncated_server_extension_is_detected (function)
macro_rules! Depcrate_msgs_handshake_testtest_truncated_server_extension_is_detected {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"test_truncated_server_extension_is_detected"}
// Dependencies: {}
# [test] fn test_truncated_server_extension_is_detected () { let shp = sample_server_hello_payload () ; let mut enc = shp . extensions . get_encoding () ; println ! ("testing enc {enc:?}") ; for l in 0 .. enc . len () { assert ! (ServerExtensions :: read_bytes (& enc [.. l]) . is_err ()) ; } for l in 0 .. (enc . len () - 4) { put_u16 (l as u16 , & mut enc [.. 2]) ; println ! ("  encoding {enc:?} len {l:?}") ; assert ! (ServerExtensions :: read_bytes (& enc) . is_err ()) ; } }
};
}
