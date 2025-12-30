// Generated macro for client_extensions_basics (function)
macro_rules! Depcrate_msgs_handshake_testclient_extensions_basics {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"client_extensions_basics"}
// Dependencies: {}
# [test] fn client_extensions_basics () { let src = ClientExtensions { early_data_request : Some (()) , .. Default :: default () } ; let mut target = ClientExtensions :: default () ; assert_eq ! (src . collect_used () , vec ! [ExtensionType :: EarlyData]) ; assert_eq ! (target . collect_used () , vec ! []) ; target . clone_one (& src , ExtensionType :: EarlyData) ; assert_eq ! (target . collect_used () , vec ! [ExtensionType :: EarlyData]) ; }
};
}
