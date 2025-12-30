// Generated macro for round4_step (function)
macro_rules! Depcrate_compressround4_step {
() => {
// Module: crate::compress
// Provides: {"round4_step"}
// Dependencies: {}
# [inline (always)] fn round4_step (a : u32 , b : & mut u32 , c : u32 , d : u32 , e : & mut u32 , mt : u32) { * e = e . wrapping_add (a . rotate_left (5) . wrapping_add (f4 (* b , c , d)) . wrapping_add (K [3]) . wrapping_add (mt) ,) ; * b = b . rotate_left (30) ; }
};
}
