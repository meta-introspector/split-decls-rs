// Generated macro for round1_step_bw (function)
macro_rules! Depcrate_compressround1_step_bw {
() => {
// Module: crate::compress
// Provides: {"round1_step_bw"}
// Dependencies: {}
# [inline (always)] fn round1_step_bw (a : u32 , b : & mut u32 , c : u32 , d : u32 , e : & mut u32 , mt : u32) { * b = b . rotate_right (30) ; * e = e . wrapping_sub (a . rotate_left (5) . wrapping_add (f1 (* b , c , d)) . wrapping_add (K [0]) . wrapping_add (mt) ,) ; }
};
}
