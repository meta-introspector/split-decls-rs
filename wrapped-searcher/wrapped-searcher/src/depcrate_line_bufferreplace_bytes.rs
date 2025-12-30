// Generated macro for replace_bytes (function)
macro_rules! Depcrate_line_bufferreplace_bytes {
() => {
// Module: crate::line_buffer
// Provides: {"replace_bytes"}
// Dependencies: {}
# [doc = " Replaces `src` with `replacement` in bytes, and return the offset of the"] # [doc = " first replacement, if one exists."] fn replace_bytes (mut bytes : & mut [u8] , src : u8 , replacement : u8 ,) -> Option < usize > { if src == replacement { return None ; } let first_pos = bytes . find_byte (src) ? ; bytes [first_pos] = replacement ; bytes = & mut bytes [first_pos + 1 ..] ; while let Some (i) = bytes . find_byte (src) { bytes [i] = replacement ; bytes = & mut bytes [i + 1 ..] ; while bytes . get (0) == Some (& src) { bytes [0] = replacement ; bytes = & mut bytes [1 ..] ; } } Some (first_pos) }
};
}
