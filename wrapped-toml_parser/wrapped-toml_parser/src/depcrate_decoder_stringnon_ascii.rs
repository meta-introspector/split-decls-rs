// Generated macro for NON_ASCII (const)
macro_rules! Depcrate_decoder_stringNON_ASCII {
() => {
// Module: crate::decoder::string
// Provides: {"NON_ASCII"}
// Dependencies: {}
# [doc = " `non-ascii = %x80-D7FF / %xE000-10FFFF`"] # [doc = " - ASCII is 0xxxxxxx"] # [doc = " - First byte for UTF-8 is 11xxxxxx"] # [doc = " - Subsequent UTF-8 bytes are 10xxxxxx"] const NON_ASCII : RangeInclusive < u8 > = 0x80 ..= 0xff ;
};
}
