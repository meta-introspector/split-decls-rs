// Generated macro for get_ech_config (function)
macro_rules! Depcrate_msgs_handshake_testget_ech_config {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"get_ech_config"}
// Dependencies: {}
fn get_ech_config (encoded : & [u8]) -> Vec < EchConfigPayload > { Vec :: < _ > :: read (& mut Reader :: init (encoded)) . unwrap () }
};
}
