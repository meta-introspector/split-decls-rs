// Generated macro for XzDecoder (struct)
macro_rules! Depcrate_bufreadXzDecoder {
() => {
// Module: crate::bufread
// Provides: {"XzDecoder"}
// Dependencies: {}
# [doc = " A xz decoder, or decompressor."] # [doc = ""] # [doc = " This structure implements a `BufRead` interface and takes a stream of"] # [doc = " compressed data as input, providing the decompressed data when read from."] pub struct XzDecoder < R > { obj : R , data : Stream , }
};
}
