// Generated macro for longest_zero_sequence (function)
macro_rules! Depcrate_hostlongest_zero_sequence {
() => {
// Module: crate::host
// Provides: {"longest_zero_sequence"}
// Dependencies: {}
fn longest_zero_sequence (pieces : & [u16 ; 8]) -> (isize , isize) { let mut longest = - 1 ; let mut longest_length = - 1 ; let mut start = - 1 ; macro_rules ! finish_sequence (($ end : expr) => { if start >= 0 { let length = $ end - start ; if length > longest_length { longest = start ; longest_length = length ; } } } ;) ; for i in 0 .. 8 { if pieces [i as usize] == 0 { if start < 0 { start = i ; } } else { finish_sequence ! (i) ; start = - 1 ; } } finish_sequence ! (8) ; if longest_length < 2 { (- 1 , - 2) } else { (longest , longest + longest_length) } }
};
}
