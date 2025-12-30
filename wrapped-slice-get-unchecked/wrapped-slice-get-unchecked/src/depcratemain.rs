// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let x = vec ! [0u8 ; 4096] ; let mut i = 0 ; while i < x . len () { let _element = unsafe { * x . get_unchecked (i) } ; i += 1 ; } }
};
}
