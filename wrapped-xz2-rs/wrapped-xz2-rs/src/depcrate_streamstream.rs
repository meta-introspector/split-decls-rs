// Generated macro for Stream (struct)
macro_rules! Depcrate_streamStream {
() => {
// Module: crate::stream
// Provides: {"Stream"}
// Dependencies: {}
# [doc = " Representation of an in-memory LZMA encoding or decoding stream."] # [doc = ""] # [doc = " Wraps the raw underlying `lzma_stream` type and provides the ability to"] # [doc = " create streams which can either decode or encode various LZMA-based formats."] pub struct Stream { raw : lzma_sys :: lzma_stream , }
};
}
