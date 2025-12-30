// Generated macro for client_extensions_decode_checks_duplicates (function)
macro_rules! Depcrate_msgs_handshake_testclient_extensions_decode_checks_duplicates {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"client_extensions_decode_checks_duplicates"}
// Dependencies: {}
# [test] fn client_extensions_decode_checks_duplicates () { ClientExtensions :: read_bytes (b"\x00\x04\x00\x2a\x00\x00") . unwrap () ; assert_eq ! (ClientExtensions :: read_bytes (b"\x00\x08\x00\x2a\x00\x00\x00\x2a\x00\x00") . unwrap_err () , InvalidMessage :: DuplicateExtension (0x002a)) ; assert_eq ! (ClientExtensions :: read_bytes (b"\x00\x08\xff\xff\x00\x00\xff\xff\x00\x00") . unwrap_err () , InvalidMessage :: DuplicateExtension (0xffff)) ; }
};
}
