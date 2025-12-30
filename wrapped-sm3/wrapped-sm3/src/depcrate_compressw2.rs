// Generated macro for w2 (function)
macro_rules! Depcrate_compressw2 {
() => {
// Module: crate::compress
// Provides: {"w2"}
// Dependencies: {}
# [inline (always)] fn w2 (x : & mut [u32 ; 16] , i : usize) -> u32 { let tw = w1 (x , i) ^ w1 (x , i - 9) ^ w1 (x , i - 3) . rotate_left (15) ; let tw = p1 (tw) ^ w1 (x , i - 13) . rotate_left (7) ^ w1 (x , i - 6) ; x [i & 0x0f] = tw ; tw }
};
}
