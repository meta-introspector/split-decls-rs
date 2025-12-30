// Generated macro for compress (function)
macro_rules! Depcrate_compresscompress {
() => {
// Module: crate::compress
// Provides: {"compress"}
// Dependencies: {}
pub (crate) fn compress (state : & mut [u64 ; 3] , raw_block : & [u8 ; 64]) { let mut block : [u64 ; 8] = Default :: default () ; for (o , chunk) in block . iter_mut () . zip (raw_block . chunks_exact (8)) { * o = u64 :: from_le_bytes (chunk . try_into () . unwrap ()) ; } let [mut a , mut b , mut c] = * state ; pass (& mut a , & mut b , & mut c , & block , 5) ; key_schedule (& mut block) ; pass (& mut c , & mut a , & mut b , & block , 7) ; key_schedule (& mut block) ; pass (& mut b , & mut c , & mut a , & block , 9) ; state [0] ^= a ; state [1] = b . wrapping_sub (state [1]) ; state [2] = c . wrapping_add (state [2]) ; }
};
}
