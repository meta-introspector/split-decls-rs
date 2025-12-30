// Generated macro for zero_pad_byte_slice (function)
macro_rules! Depcrate_pkezero_pad_byte_slice {
() => {
// Module: crate::pke
// Provides: {"zero_pad_byte_slice"}
// Dependencies: {}
# [doc = " Converts a byte slice to a fixed-size array, padding with leading zeroes if necessary."] pub (crate) fn zero_pad_byte_slice < const N : usize > (bytes : & [u8]) -> der :: Result < [u8 ; N] > { let num_zeroes = N . checked_sub (bytes . len ()) . ok_or_else (| | Tag :: Integer . length_error ()) ? ; let mut output = [0u8 ; N] ; output [num_zeroes ..] . copy_from_slice (bytes) ; Ok (output) }
};
}
