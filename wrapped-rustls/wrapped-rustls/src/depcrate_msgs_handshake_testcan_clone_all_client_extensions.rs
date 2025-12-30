// Generated macro for can_clone_all_client_extensions (function)
macro_rules! Depcrate_msgs_handshake_testcan_clone_all_client_extensions {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"can_clone_all_client_extensions"}
// Dependencies: {}
# [test] fn can_clone_all_client_extensions () { let exts = sample_client_hello_payload () . extensions ; let exts2 = exts . clone () ; println ! ("{exts:?}, {exts2:?}") ; }
};
}
