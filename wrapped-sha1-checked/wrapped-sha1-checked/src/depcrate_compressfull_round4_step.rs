// Generated macro for full_round4_step (function)
macro_rules! Depcrate_compressfull_round4_step {
() => {
// Module: crate::compress
// Provides: {"full_round4_step"}
// Dependencies: {}
# [inline (always)] fn full_round4_step (a : u32 , b : & mut u32 , c : u32 , d : u32 , e : & mut u32 , w : & mut [u32 ; 80] , t : usize) { w [t] = mix (w , t) ; * e = e . wrapping_add (w [t] . wrapping_add (a . rotate_left (5)) . wrapping_add (f4 (* b , c , d)) . wrapping_add (K [3]) ,) ; * b = b . rotate_left (30) ; }
};
}
