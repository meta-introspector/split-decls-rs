// Generated macro for read_varint_meta2 (function)
macro_rules! Depcrate_varintread_varint_meta2 {
() => {
// Module: crate::varint
// Provides: {"read_varint_meta2"}
// Dependencies: {}
# [doc = " Reads a varint with 2 bits of metadata in the lead byte."] # [doc = ""] # [doc = " Returns the varint value and a subslice of `remainder` with the varint bytes removed."] # [doc = ""] # [doc = " If the varint spills off the end of the slice, a debug assertion will fail,"] # [doc = " and the function will return the value up to that point."] pub const fn read_varint_meta2 (start : u8 , remainder : & [u8]) -> (usize , & [u8]) { let mut value = (start & 0b00011111) as usize ; let mut remainder = remainder ; if (start & 0b00100000) != 0 { loop { let next ; (next , remainder) = debug_unwrap ! (remainder . split_first () , break , "invalid varint") ; value = (value << 7) + ((* next & 0b01111111) as usize) + 32 ; if (* next & 0b10000000) == 0 { break ; } } } (value , remainder) }
};
}
