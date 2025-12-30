// Generated macro for can_decode_server_hello_from_api_devicecheck_apple_com (function)
macro_rules! Depcrate_msgs_handshake_testcan_decode_server_hello_from_api_devicecheck_apple_com {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"can_decode_server_hello_from_api_devicecheck_apple_com"}
// Dependencies: {}
# [test] fn can_decode_server_hello_from_api_devicecheck_apple_com () { let data = include_bytes ! ("../testdata/hello-api.devicecheck.apple.com.bin") ; let mut r = Reader :: init (data) ; let hm = HandshakeMessagePayload :: read (& mut r) . unwrap () ; println ! ("msg: {hm:?}") ; }
};
}
