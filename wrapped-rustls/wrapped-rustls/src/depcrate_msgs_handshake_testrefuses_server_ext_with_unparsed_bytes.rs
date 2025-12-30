// Generated macro for refuses_server_ext_with_unparsed_bytes (function)
macro_rules! Depcrate_msgs_handshake_testrefuses_server_ext_with_unparsed_bytes {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"refuses_server_ext_with_unparsed_bytes"}
// Dependencies: {}
# [test] fn refuses_server_ext_with_unparsed_bytes () { let bytes = [0x00u8 , 0x08 , 0x00 , 0x0b , 0x00 , 0x04 , 0x02 , 0xf8 , 0x01 , 0x02] ; assert_eq ! (ServerExtensions :: read_bytes (& bytes) . unwrap_err () , InvalidMessage :: TrailingData ("ServerExtensions")) ; }
};
}
