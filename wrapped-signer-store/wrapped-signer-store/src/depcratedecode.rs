// Generated macro for decode (function)
macro_rules! Depcratedecode {
() => {
// Module: crate
// Provides: {"decode"}
// Dependencies: {}
# [doc = " Decodes an encoded byte slice into one or two boolean vectors."] # [doc = ""] # [doc = " It reads the first byte to determine the encoding scheme and then decodes"] # [doc = " the rest of the data accordingly."] pub fn decode (bytes : & [u8] , max_len : usize) -> Result < Decoded , DecodeError > { if bytes . len () < 3 { return Err (DecodeError :: InputTooShort) ; } let version_byte = bytes [0] ; let version = Version :: from_u8 (version_byte) . ok_or (DecodeError :: UnsupportedEncoding) ? ; let mut len_arr = [0u8 ; 2] ; len_arr . copy_from_slice (& bytes [1 .. 3]) ; let total_bits = u16 :: from_le_bytes (len_arr) as usize ; if total_bits > max_len { return Err (DecodeError :: CorruptDataPayload) ; } let data_bytes = & bytes [3 ..] ; match version { Version :: Base2 => decode_impl_base2 (data_bytes , total_bits) , Version :: Base3 => decode_impl_base3 (data_bytes , total_bits) , } }
};
}
