// Generated macro for LengthPrefixedBuffer (struct)
macro_rules! Depcrate_msgs_codecLengthPrefixedBuffer {
() => {
// Module: crate::msgs::codec
// Provides: {"LengthPrefixedBuffer"}
// Dependencies: {}
# [doc = " Tracks encoding a length-delimited structure in a single pass."] pub (crate) struct LengthPrefixedBuffer < 'a > { pub (crate) buf : & 'a mut Vec < u8 > , len_offset : usize , size_len : ListLength , }
};
}
