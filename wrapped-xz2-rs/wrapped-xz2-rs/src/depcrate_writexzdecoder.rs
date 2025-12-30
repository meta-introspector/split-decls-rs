// Generated macro for XzDecoder (struct)
macro_rules! Depcrate_writeXzDecoder {
() => {
// Module: crate::write
// Provides: {"XzDecoder"}
// Dependencies: {}
# [doc = " A compression stream which will have compressed data written to it and"] # [doc = " will write uncompressed data to an output stream."] pub struct XzDecoder < W : Write > { data : Stream , obj : Option < W > , buf : Vec < u8 > , }
};
}
