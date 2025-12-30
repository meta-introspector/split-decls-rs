// Generated macro for reads_random (function)
macro_rules! Depcrate_msgs_handshake_testreads_random {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"reads_random"}
// Dependencies: {}
# [test] fn reads_random () { let bytes = [0x01 ; 32] ; let mut rd = Reader :: init (& bytes) ; let rnd = Random :: read (& mut rd) . unwrap () ; println ! ("{rnd:?}") ; assert ! (! rd . any_left ()) ; }
};
}
