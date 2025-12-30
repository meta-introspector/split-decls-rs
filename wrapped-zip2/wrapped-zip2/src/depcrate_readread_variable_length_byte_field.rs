// Generated macro for read_variable_length_byte_field (function)
macro_rules! Depcrate_readread_variable_length_byte_field {
() => {
// Module: crate::read
// Provides: {"read_variable_length_byte_field"}
// Dependencies: {}
# [inline] fn read_variable_length_byte_field < R : Read > (reader : & mut R , len : usize) -> io :: Result < Box < [u8] > > { let mut data = vec ! [0 ; len] . into_boxed_slice () ; reader . read_exact (& mut data) ? ; Ok (data) }
};
}
