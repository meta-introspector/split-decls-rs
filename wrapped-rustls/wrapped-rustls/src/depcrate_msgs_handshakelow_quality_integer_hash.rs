// Generated macro for low_quality_integer_hash (function)
macro_rules! Depcrate_msgs_handshakelow_quality_integer_hash {
() => {
// Module: crate::msgs::handshake
// Provides: {"low_quality_integer_hash"}
// Dependencies: {}
fn low_quality_integer_hash (mut x : u32) -> u32 { x = x . wrapping_add (0x7ed55d16) . wrapping_add (x << 12) ; x = (x ^ 0xc761c23c) ^ (x >> 19) ; x = x . wrapping_add (0x165667b1) . wrapping_add (x << 5) ; x = x . wrapping_add (0xd3a2646c) ^ (x << 9) ; x = x . wrapping_add (0xfd7046c5) . wrapping_add (x << 3) ; x = (x ^ 0xb55a4f09) ^ (x >> 16) ; x }
};
}
