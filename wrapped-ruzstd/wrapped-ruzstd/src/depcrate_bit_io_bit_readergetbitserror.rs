// Generated macro for GetBitsError (enum)
macro_rules! Depcrate_bit_io_bit_readerGetBitsError {
() => {
// Module: crate::bit_io::bit_reader
// Provides: {"GetBitsError"}
// Dependencies: {}
# [derive (Debug)] # [non_exhaustive] pub enum GetBitsError { TooManyBits { num_requested_bits : usize , limit : u8 , } , NotEnoughRemainingBits { requested : usize , remaining : usize , } , }
};
}
