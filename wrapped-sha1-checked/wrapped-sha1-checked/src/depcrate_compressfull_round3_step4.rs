// Generated macro for full_round3_step4 (function)
macro_rules! Depcrate_compressfull_round3_step4 {
() => {
// Module: crate::compress
// Provides: {"full_round3_step4"}
// Dependencies: {}
# [inline] fn full_round3_step4 (a : & mut u32 , b : & mut u32 , c : & mut u32 , d : & mut u32 , e : & mut u32 , w : & mut [u32 ; 80] , t : usize ,) { w [t] = mix (w , t) ; w [t + 1] = mix (w , t + 1) ; w [t + 2] = mix (w , t + 2) ; w [t + 3] = mix (w , t + 3) ; round3_step4 (a , b , c , d , e , w , t) ; }
};
}
