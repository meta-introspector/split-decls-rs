// Generated macro for FrameHeader (struct)
macro_rules! Depcrate_decoding_frameFrameHeader {
() => {
// Module: crate::decoding::frame
// Provides: {"FrameHeader"}
// Dependencies: {}
# [doc = " A frame header has a variable size, with a minimum of 2 bytes, and a maximum of 14 bytes."] pub struct FrameHeader { pub descriptor : FrameDescriptor , # [doc = " The `Window_Descriptor` field contains the minimum size of a memory buffer needed to"] # [doc = " decompress the entire frame."] # [doc = ""] # [doc = " This byte is not included in the frame header when the `Single_Segment_flag` is set."] # [doc = ""] # [doc = " Bits 7-3 refer to the `Exponent`, where bits 2-0 refer to the `Mantissa`."] # [doc = ""] # [doc = " To determine the size of a window, the following formula can be used:"] # [doc = " ```text"] # [doc = " windowLog = 10 + Exponent;"] # [doc = " windowBase = 1 << windowLog;"] # [doc = " windowAdd = (windowBase / 8) * Mantissa;"] # [doc = " Window_Size = windowBase + windowAdd;"] # [doc = " ```"] # [doc = " <https://github.com/facebook/zstd/blob/dev/doc/zstd_compression_format.md#window_descriptor>"] window_descriptor : u8 , # [doc = " The `Dictionary_ID` field contains the ID of the dictionary to be used to decode the frame."] # [doc = " When this value is not present, it's up to the decoder to know which dictionary to use."] dict_id : Option < u32 > , # [doc = " The size of the original/uncompressed content."] frame_content_size : u64 , }
};
}
