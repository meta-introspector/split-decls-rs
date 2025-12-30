// Generated macro for bytes_for_varint (function)
macro_rules! Depcrate_identifierbytes_for_varint {
() => {
// Module: crate::identifier
// Provides: {"bytes_for_varint"}
// Dependencies: {}
fn bytes_for_varint (len : NonZeroUsize) -> usize { let usize_bits = mem :: size_of :: < usize > () * 8 ; let len_bits = usize_bits - len . leading_zeros () as usize ; (len_bits + 6) / 7 }
};
}
