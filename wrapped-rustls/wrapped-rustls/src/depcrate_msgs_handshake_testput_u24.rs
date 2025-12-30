// Generated macro for put_u24 (function)
macro_rules! Depcrate_msgs_handshake_testput_u24 {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"put_u24"}
// Dependencies: {}
fn put_u24 (u : u32 , b : & mut [u8]) { b [0] = (u >> 16) as u8 ; b [1] = (u >> 8) as u8 ; b [2] = u as u8 ; }
};
}
