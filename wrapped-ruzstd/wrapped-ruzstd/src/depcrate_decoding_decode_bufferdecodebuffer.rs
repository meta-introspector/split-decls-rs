// Generated macro for DecodeBuffer (struct)
macro_rules! Depcrate_decoding_decode_bufferDecodeBuffer {
() => {
// Module: crate::decoding::decode_buffer
// Provides: {"DecodeBuffer"}
// Dependencies: {}
pub struct DecodeBuffer { buffer : RingBuffer , pub dict_content : Vec < u8 > , pub window_size : usize , total_output_counter : u64 , # [cfg (feature = "hash")] pub hash : twox_hash :: XxHash64 , }
};
}
