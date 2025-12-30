// Generated macro for inv_mix (function)
macro_rules! Depcrateinv_mix {
() => {
// Module: crate
// Provides: {"inv_mix"}
// Dependencies: {}
fn inv_mix (r : u32 , y : (u64 , u64)) -> (u64 , u64) { let x1 = (y . 0 ^ y . 1) . rotate_right (r) ; let x0 = y . 0 . wrapping_sub (x1) ; (x0 , x1) }
};
}
