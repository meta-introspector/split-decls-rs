// Generated macro for client_extensions_empty (function)
macro_rules! Depcrate_msgs_handshake_testclient_extensions_empty {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"client_extensions_empty"}
// Dependencies: {}
# [test] fn client_extensions_empty () { assert_eq ! (ClientExtensions :: default () . get_encoding () , Vec ::< u8 >:: new ()) ; assert_eq ! (ClientExtensions :: read_bytes (& []) . unwrap () . collect_used () , vec ! []) ; let early_data = b"\x00\x04\x00\x2a\x00\x00" ; assert_eq ! (ClientExtensions { early_data_request : Some (()) , .. Default :: default () } . get_encoding () , early_data) ; assert_eq ! (ClientExtensions :: read_bytes (early_data) . unwrap () . collect_used () , vec ! [ExtensionType :: EarlyData]) ; }
};
}
