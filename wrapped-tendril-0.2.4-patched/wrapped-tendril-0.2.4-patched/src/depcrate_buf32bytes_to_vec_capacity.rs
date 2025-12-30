// Generated macro for bytes_to_vec_capacity (function)
macro_rules! Depcrate_buf32bytes_to_vec_capacity {
() => {
// Module: crate::buf32
// Provides: {"bytes_to_vec_capacity"}
// Dependencies: {}
# [inline (always)] fn bytes_to_vec_capacity < H > (x : u32) -> usize { let header = mem :: size_of :: < H > () ; debug_assert ! (header > 0) ; let x = (x as usize) . checked_add (header) . expect (OFLOW) ; 1 + ((x - 1) / header) }
};
}
