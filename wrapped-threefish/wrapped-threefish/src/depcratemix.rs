// Generated macro for mix (function)
macro_rules! Depcratemix {
() => {
// Module: crate
// Provides: {"mix"}
// Dependencies: {}
fn mix (r : u32 , x : (u64 , u64)) -> (u64 , u64) { let y0 = x . 0 . wrapping_add (x . 1) ; let y1 = x . 1 . rotate_left (r) ^ y0 ; (y0 , y1) }
};
}
