// Generated macro for write_varint_meta3 (function)
macro_rules! Depcrate_varintwrite_varint_meta3 {
() => {
// Module: crate::varint
// Provides: {"write_varint_meta3"}
// Dependencies: {}
# [doc = " Returns a new [`ConstArrayBuilder`] containing a varint with 3 bits of metadata."] # [allow (clippy :: indexing_slicing)] pub (crate) const fn write_varint_meta3 (value : usize) -> ConstArrayBuilder < MAX_VARINT_LENGTH , u8 > { let mut result = [0 ; MAX_VARINT_LENGTH] ; let mut i = MAX_VARINT_LENGTH - 1 ; let mut value = value ; let mut last = true ; loop { if value < 16 { result [i] = value as u8 ; if ! last { result [i] |= 0b00010000 ; } break ; } value -= 16 ; result [i] = (value as u8) & 0b01111111 ; if ! last { result [i] |= 0b10000000 ; } else { last = false ; } value >>= 7 ; i -= 1 ; } ConstArrayBuilder :: from_manual_slice (result , i , MAX_VARINT_LENGTH) }
};
}
