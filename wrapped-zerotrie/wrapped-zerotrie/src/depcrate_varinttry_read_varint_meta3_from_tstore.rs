// Generated macro for try_read_varint_meta3_from_tstore (function)
macro_rules! Depcrate_varinttry_read_varint_meta3_from_tstore {
() => {
// Module: crate::varint
// Provides: {"try_read_varint_meta3_from_tstore"}
// Dependencies: {}
# [doc = " Reads and removes a varint with 3 bits of metadata from a [`TrieBuilderStore`]."] # [doc = ""] # [doc = " Returns the varint value."] # [cfg (feature = "alloc")] pub (crate) fn try_read_varint_meta3_from_tstore < S : TrieBuilderStore > (start : u8 , remainder : & mut S ,) -> Option < usize > { let mut value = (start & 0b00001111) as usize ; if (start & 0b00010000) != 0 { loop { let next = remainder . atbs_pop_front () ? ; value = (value << 7) + ((next & 0b01111111) as usize) + 16 ; if (next & 0b10000000) == 0 { break ; } } } Some (value) }
};
}
