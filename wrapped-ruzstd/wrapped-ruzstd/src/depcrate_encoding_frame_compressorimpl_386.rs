// Generated macro for impl_386 (impl)
macro_rules! Depcrate_encoding_frame_compressorimpl_386 {
() => {
// Module: crate::encoding::frame_compressor
// Provides: {"impl_386"}
// Dependencies: {}
impl < R : Read , W : Write > FrameCompressor < R , W , MatchGeneratorDriver > { # [doc = " Create a new `FrameCompressor`"] pub fn new (compression_level : CompressionLevel) -> Self { Self { uncompressed_data : None , compressed_data : None , compression_level , state : CompressState { matcher : MatchGeneratorDriver :: new (1024 * 128 , 1) , last_huff_table : None , fse_tables : FseTables :: new () , } , # [cfg (feature = "hash")] hasher : XxHash64 :: with_seed (0) , } } }
};
}
