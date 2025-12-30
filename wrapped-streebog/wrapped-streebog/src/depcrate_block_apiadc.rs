// Generated macro for adc (function)
macro_rules! Depcrate_block_apiadc {
() => {
// Module: crate::block_api
// Provides: {"adc"}
// Dependencies: {}
# [inline (always)] fn adc (v1 : & mut u64 , v2 : u64 , carry : & mut bool) { let (a , b) = v1 . overflowing_add (v2) ; let (c , d) = a . overflowing_add (* carry as u64) ; * v1 = c ; * carry = b || d ; }
};
}
