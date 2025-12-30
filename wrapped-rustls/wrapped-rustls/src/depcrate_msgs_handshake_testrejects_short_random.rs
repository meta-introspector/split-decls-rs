// Generated macro for rejects_short_random (function)
macro_rules! Depcrate_msgs_handshake_testrejects_short_random {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"rejects_short_random"}
// Dependencies: {}
# [test] fn rejects_short_random () { let bytes = [0x01 ; 31] ; let mut rd = Reader :: init (& bytes) ; assert ! (Random :: read (& mut rd) . is_err ()) ; }
};
}
