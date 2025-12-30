// Generated macro for round3_step (function)
macro_rules! Depcrate_compressround3_step {
() => {
// Module: crate::compress
// Provides: {"round3_step"}
// Dependencies: {}
# [inline (always)] fn round3_step (a : u32 , b : & mut u32 , c : u32 , d : u32 , e : & mut u32 , mt : u32) { * e = e . wrapping_add (a . rotate_left (5) . wrapping_add (f3 (* b , c , d)) . wrapping_add (K [2]) . wrapping_add (mt) ,) ; * b = b . rotate_left (30) ; }
};
}
