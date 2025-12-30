// Generated macro for full_round3_step (function)
macro_rules! Depcrate_compressfull_round3_step {
() => {
// Module: crate::compress
// Provides: {"full_round3_step"}
// Dependencies: {}
# [inline (always)] fn full_round3_step (a : u32 , b : & mut u32 , c : u32 , d : u32 , e : & mut u32 , w : & mut [u32 ; 80] , t : usize) { w [t] = mix (w , t) ; * e = e . wrapping_add (w [t] . wrapping_add (a . rotate_left (5)) . wrapping_add (f3 (* b , c , d)) . wrapping_add (K [2]) ,) ; * b = b . rotate_left (30) ; }
};
}
