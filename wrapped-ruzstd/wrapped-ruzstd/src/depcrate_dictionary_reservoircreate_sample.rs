// Generated macro for create_sample (function)
macro_rules! Depcrate_dictionary_reservoircreate_sample {
() => {
// Module: crate::dictionary::reservoir
// Provides: {"create_sample"}
// Dependencies: {}
# [doc = " Creates a representative sample of `input` of `size` bytes."] pub fn create_sample < R : io :: Read > (input : & mut R , size : usize) -> Vec < u8 > { let reservoir = Reservoir :: new (size) ; reservoir . fill (input) }
};
}
