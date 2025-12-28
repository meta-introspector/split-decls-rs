macro_rules! deps {
    () => {
        Read!();
        CompressState!();
        MatchGeneratorDriver!();
        Write!();
        FseTables!();
        CompressionLevel!();
        FrameCompressor!();
    };
}

macro_rules! impl_266 {
    () => {
        deps!();
        impl < R : Read , W : Write > FrameCompressor < R , W , MatchGeneratorDriver > { # [doc = " Create a new `FrameCompressor`"] pub fn new (compression_level : CompressionLevel) -> Self { Self { uncompressed_data : None , compressed_data : None , compression_level , state : CompressState { matcher : MatchGeneratorDriver :: new (1024 * 128 , 1) , last_huff_table : None , fse_tables : FseTables :: new () , } , # [cfg (feature = "hash")] hasher : XxHash64 :: with_seed (0) , } } }
    };
}

impl_266!();