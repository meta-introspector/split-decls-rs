macro_rules! OFFSET_DEFAULT_DISTRIBUTION {
    () => {
        # [doc = " If [ModeType::Predefined] is selected for a symbol type, its FSE decoding"] # [doc = " table is generated using a predefined distribution table."] # [doc = ""] # [doc = " https://github.com/facebook/zstd/blob/dev/doc/zstd_compression_format.md#match-length"] const OFFSET_DEFAULT_DISTRIBUTION : [i32 ; 29] = [1 , 1 , 1 , 1 , 1 , 1 , 2 , 2 , 2 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , - 1 , - 1 , - 1 , - 1 , - 1 ,] ;
    };
}

OFFSET_DEFAULT_DISTRIBUTION!();