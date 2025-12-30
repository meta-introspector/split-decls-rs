// Generated macro for full_round1_step_load4 (function)
macro_rules! Depcrate_compressfull_round1_step_load4 {
() => {
// Module: crate::compress
// Provides: {"full_round1_step_load4"}
// Dependencies: {}
# [inline] fn full_round1_step_load4 (a : & mut u32 , b : & mut u32 , c : & mut u32 , d : & mut u32 , e : & mut u32 , m : & [u32 ; 16] , w : & mut [u32 ; 80] , t : usize ,) { w [t .. t + 4] . copy_from_slice (& m [t .. t + 4]) ; round1_step4 (a , b , c , d , e , w , t) ; }
};
}
