macro_rules! deps {
    () => {
        Write!();
        Read!();
        CompressionLevel!();
        Matcher!();
        CompressState!();
    };
}

macro_rules! FrameCompressor {
    () => {
        deps!();
        # [doc = " An interface for compressing arbitrary data with the ZStandard compression algorithm."] # [doc = ""] # [doc = " `FrameCompressor` will generally be used by:"] # [doc = " 1. Initializing a compressor by providing a buffer of data using `FrameCompressor::new()`"] # [doc = " 2. Starting compression and writing that compression into a vec using `FrameCompressor::begin`"] # [doc = ""] # [doc = " # Examples"] # [doc = " ```"] # [doc = " use ruzstd::encoding::{FrameCompressor, CompressionLevel};"] # [doc = " let mock_data: &[_] = &[0x1, 0x2, 0x3, 0x4];"] # [doc = " let mut output = std::vec::Vec::new();"] # [doc = " // Initialize a compressor."] # [doc = " let mut compressor = FrameCompressor::new(CompressionLevel::Uncompressed);"] # [doc = " compressor.set_source(mock_data);"] # [doc = " compressor.set_drain(&mut output);"] # [doc = ""] # [doc = " // `compress` writes the compressed output into the provided buffer."] # [doc = " compressor.compress();"] # [doc = " ```"] pub struct FrameCompressor < R : Read , W : Write , M : Matcher > { uncompressed_data : Option < R > , compressed_data : Option < W > , compression_level : CompressionLevel , state : CompressState < M > , # [cfg (feature = "hash")] hasher : XxHash64 , }
    };
}

FrameCompressor!();