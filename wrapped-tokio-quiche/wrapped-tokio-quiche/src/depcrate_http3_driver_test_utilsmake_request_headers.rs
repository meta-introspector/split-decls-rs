// Generated macro for make_request_headers (function)
macro_rules! Depcrate_http3_driver_test_utilsmake_request_headers {
() => {
// Module: crate::http3::driver::test_utils
// Provides: {"make_request_headers"}
// Dependencies: {}
pub fn make_request_headers (method : & str) -> Vec < Header > { vec ! [Header :: new (b":method" , method . as_bytes ()) , Header :: new (b":scheme" , b"https") , Header :: new (b":authority" , b"quic.tech") , Header :: new (b":path" , b"/test") ,] }
};
}
