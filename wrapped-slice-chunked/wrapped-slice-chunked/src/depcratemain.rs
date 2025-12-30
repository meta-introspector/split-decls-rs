// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let data_len = 2 * N ; let mut rgba_data = Vec :: with_capacity (data_len) ; let img_data = input_vec () ; for slice in img_data . chunks (2) { let gray = slice [0] ; let alpha = slice [1] ; rgba_data . push (gray) ; rgba_data . push (gray) ; rgba_data . push (gray) ; rgba_data . push (alpha) ; } assert_eq ! (rgba_data . len () , data_len) ; }
};
}
