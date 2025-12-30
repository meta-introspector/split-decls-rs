// Generated macro for XzEncoder (struct)
macro_rules! Depcrate_bufreadXzEncoder {
() => {
// Module: crate::bufread
// Provides: {"XzEncoder"}
// Dependencies: {}
# [doc = " An xz encoder, or compressor."] # [doc = ""] # [doc = " This structure implements a `BufRead` interface and will read uncompressed"] # [doc = " data from an underlying stream and emit a stream of compressed data."] pub struct XzEncoder < R > { obj : R , data : Stream , }
};
}
